use exif::{DateTime, Exif, In, Tag};
use serde::Serialize;
use std::fs::ReadDir;
use std::io::BufReader;

fn parse_coordinates(exif: &Exif, tag: Tag, r#ref: Tag) -> Option<f32> {
    let mut coord = None;
    let mut coord_ref = None;

    // Parse DMS coordinates
    if let Some(field) = exif.get_field(tag, In::PRIMARY) {
        match field.value {
            exif::Value::Rational(ref vec) if !vec.is_empty() => {
                let deg = vec[0].to_f64() as u16;
                let min = vec[1].to_f64() as u8;
                let sec = vec[2].to_f64();

                coord = Some((deg, min, sec));
            }
            _ => {}
        }
    }

    // Get bearing
    if let Some(field) = exif.get_field(r#ref, In::PRIMARY) {
        coord_ref = Some(field.display_value().to_string());
    }

    match (coord, coord_ref) {
        (Some((deg, min, sec)), Some(r#ref)) => {
            use dms_coordinates::Cardinal;
            use dms_coordinates::Cardinal::*;
            let bearing: Option<Cardinal> = match r#ref.as_str() {
                "N" => Some(North),
                "NE" => Some(NorthEast),
                "NW" => Some(NorthWest),
                "S" => Some(South),
                "SE" => Some(SouthEast),
                "SW" => Some(SouthWest),
                "E" => Some(East),
                "W" => Some(West),
                _ => None,
            };

            let dms = dms_coordinates::DMS::new(deg, min, sec, bearing);

            Some(dms.to_ddeg_angle() as f32)
        }
        (_, _) => None,
    }
}

#[derive(Debug, Serialize)]
struct Metadata {
    filename: String,
    date: String,
    width: u32,
    height: u32,
    gps: Gps,
    location: Option<Location>,
}

#[derive(Debug, Serialize)]
pub struct Location {
    precise: String,
    broad: String,
}

#[derive(Debug, Serialize)]
pub struct Gps {
    latitude: f32,
    longitude: f32,
}

#[derive(Debug, Serialize)]
pub struct MetadataList(Vec<Metadata>);

impl MetadataList {
    pub fn process_folder(dir: ReadDir, get_location: bool) -> Self {
        let mut files = vec![];

        for file in dir {
            let Ok(file) = file else {
                continue;
            };

            // Get filename
            let Ok(filename) = file.file_name().into_string() else {
                continue;
            };

            // Read exif from file
            let Ok(file) = std::fs::File::open(file.path()) else {
                continue;
            };
            let mut reader = BufReader::new(file);
            let Ok(exif) = exif::Reader::new().read_from_container(&mut reader) else {
                continue;
            };
            eprintln!("Processing `{}`", filename);

            // Get GPS coordinates
            let latitude = parse_coordinates(&exif, Tag::GPSLatitude, Tag::GPSLatitudeRef);
            let longitude = parse_coordinates(&exif, Tag::GPSLongitude, Tag::GPSLongitudeRef);

            // Get date
            let mut date = None;
            if let Some(field) = exif.get_field(Tag::DateTime, In::PRIMARY) {
                match field.value {
                    exif::Value::Ascii(ref vec) if !vec.is_empty() => {
                        if let Ok(datetime) = DateTime::from_ascii(&vec[0]) {
                            let datetime = datetime.to_string();
                            let split: Vec<&str> = datetime.split(' ').collect();

                            date = split.first().map(|str| str.to_string());
                        }
                    }
                    _ => {}
                }
            }

            // Get image width
            let mut width = None;
            if let Some(field) = exif.get_field(Tag::PixelXDimension, In::PRIMARY) {
                if let Some(exif_width) = field.value.get_uint(0) {
                    width = Some(exif_width);
                }
            }

            // Get image height
            let mut height = None;
            if let Some(field) = exif.get_field(Tag::PixelYDimension, In::PRIMARY) {
                if let Some(exif_height) = field.value.get_uint(0) {
                    height = Some(exif_height);
                }
            }

            match (date, latitude, longitude, width, height) {
                (Some(date), Some(latitude), Some(longitude), Some(width), Some(height)) => {
                    let gps = Gps {
                        latitude,
                        longitude,
                    };
                    let location = if get_location {
                        eprintln!("Getting location for `{}`", filename);
                        gps.get_location()
                    } else {
                        None
                    };
                    files.push(Metadata {
                        filename,
                        width,
                        height,
                        date,
                        gps,
                        location,
                    });
                }
                _ => {
                    continue;
                }
            }
        }

        Self(files)
    }

    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string(&self.0)
    }
}

impl Gps {
    fn get_location(&self) -> Option<Location> {
        match reqwest::blocking::Client::new()
            .get(format!(
                // Documentation: https://nominatim.org/release-docs/develop/api/Reverse/
                "https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat={}&lon={}&zoom=8",
                self.latitude, self.longitude,
            ))
            .header("User-Agent", "https://philippeloctaux.com")
            .send()
            .and_then(|data| data.json::<serde_json::Value>())
        {
            Ok(data) => {
                let location = &data["display_name"];

                if location.is_string() {
                    let location = location.to_string();
                    // Remove first and last characters (the string is wrapped in double quotes '"')
                    let location = {
                        let mut chars = location.chars();
                        chars.next();
                        chars.next_back();
                        chars.as_str()
                    };
                    eprintln!("Raw location is `{}`", location);

                    let mut location = location.split(',');
                    let precise = location.next().unwrap_or("?").to_string();

                    let mut broad: String =
                        location.collect::<Vec<&str>>().join(",").trim().to_string();
                    if broad.is_empty() {
                        broad.push('?');
                    }

                    let location = Location { precise, broad };
                    eprintln!("Location is `{:?}`", location);
                    Some(location)
                } else {
                    eprintln!("Failed to find location.");
                    None
                }
            }
            Err(_) => {
                eprintln!("Failed to make API call to get location.");
                None
            }
        }
    }
}

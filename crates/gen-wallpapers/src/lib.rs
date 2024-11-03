mod location;

use location::Gps;
use location::Location;
use location::parse_coordinates;

use exif::{DateTime, Exif, In, Tag};
use serde::Serialize;
use std::fs::ReadDir;
use std::io::BufReader;


#[derive(Debug, Serialize)]
pub struct Metadata {
    pub filename: String,
    pub date: String,
    pub width: u32,
    pub height: u32,
    pub gps: Gps,
    pub location: Option<Location>,
}

#[derive(Debug, Serialize)]
pub struct MetadataList(pub Vec<Metadata>);

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
            if let Some(field) = exif.get_field(Tag::DateTimeOriginal, In::PRIMARY) {
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
            // TODO: get OffsetTimeOriginal as well
            // TODO: use crate `jiff` to store dates with timezone offsets

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
                    let gps = Gps::new(latitude,longitude);
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
                        date: date.to_string(),
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

    pub fn to_pretty_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(&self.0)
    }
}

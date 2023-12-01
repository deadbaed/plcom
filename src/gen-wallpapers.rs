use dms_coordinates::Bearing;
use exif::{DateTime, Exif, In, Tag};
use std::fs::read_dir;
use std::io::{BufReader, Write};
use std::path::PathBuf;

const WALLPAPERS_PATH: &str = "/public/wallpapers";
const CRATE_PATH: &str = env!("CARGO_MANIFEST_DIR");

const IMPORTS: &str = r#"use crate::types::{Gps, Location, Wallpaper};"#;

#[derive(Debug)]
struct Metadata {
    file: String,
    date: String,
    latitude: f32,
    longitude: f32,
}

fn parse_coordinates(exif: &Exif, tag: Tag, r#ref: Tag) -> Option<f32> {
    let mut coord = None;
    let mut coord_ref = None;

    // Parse DMS coordinates
    if let Some(field) = exif.get_field(tag, In::PRIMARY) {
        match field.value {
            exif::Value::Rational(ref vec) if !vec.is_empty() => {
                let deg = vec[0].to_f64() as i32;
                let min = vec[1].to_f64() as i32;
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
            use Bearing::*;
            let bearing = match r#ref.as_str() {
                "N" => North,
                "NE" => NorthEast,
                "NW" => NorthWest,
                "S" => South,
                "SE" => SouthEast,
                "SW" => SouthWest,
                "E" => East,
                "W" => West,
                _ => return None,
            };

            let dms = dms_coordinates::DMS::new(deg, min, sec, bearing);

            Some(dms.to_decimal_degrees() as f32)
        }
        (_, _) => None,
    }
}

fn main() {
    let mut wallpapers = format!(
        "// AUTO GENERATED FILE.\n// PLEASE DO NOT EDIT MANUALLY.\n{IMPORTS}\npub static WALLPAPERS: &[&Wallpaper] = &[\n"
    );

    // Get list of files
    let local_wallpaper_path = format!("{}{}", CRATE_PATH, WALLPAPERS_PATH);
    let metadata: Vec<Metadata> = match read_dir(local_wallpaper_path) {
        Ok(dir) => {
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

                match (date, latitude, longitude) {
                    (Some(date), Some(latitude), Some(longitude)) => files.push(Metadata {
                        file: format!("/wallpapers/{}", filename),
                        date,
                        latitude,
                        longitude,
                    }),
                    (_, _, _) => {
                        continue;
                    }
                }
            }

            files
        }
        Err(_) => vec![],
    };

    for wallpaper in metadata {
        println!("\nProcessing `{}`", wallpaper.file);

        let client = reqwest::blocking::Client::new();
        let (precise, broad) = match client
            .get(format!(
                // Documentation: https://nominatim.org/release-docs/develop/api/Reverse/
                "https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat={}&lon={}&zoom=8",
                wallpaper.latitude, wallpaper.longitude,
            ))
            .header("User-Agent", "https://philippeloctaux.com")
            .send()
            .and_then(|data| data.json::<serde_json::Value>())
        {
            Ok(data) => {
                let location = &data["display_name"];

                let (precise, broad) = if location.is_string() {
                    let location = location.to_string();
                    // Remove first and last characters (the string is wrapped in double quotes '"')
                    let location = {
                        let mut chars = location.chars();
                        chars.next();
                        chars.next_back();
                        chars.as_str()
                    };
                    println!("Raw location is `{}`", location);

                    let mut location = location.split(',');
                    let precise = location.next().unwrap_or("?").to_string();

                    let mut broad: String =
                        location.collect::<Vec<&str>>().join(",").trim().to_string();
                    if broad.is_empty() {
                        broad.push('?');
                    }

                    (precise, broad)
                } else {
                    println!("Failed to find location.");
                    ("?".into(), "?".into())
                };

                (precise, broad)
            }
            Err(_) => {
                println!("Failed to make API call to get location.");
                ("?".into(), "?".into())
            }
        };

        println!("Precise location is `{}`", precise);
        println!("Broad location is `{}`", broad);

        // Construct structs
        let gps_struct = format!(
            "Gps {{ latitude: {}, longitude: {} }}",
            wallpaper.latitude, wallpaper.longitude
        );
        let location_struct = format!(
            "Location {{ precise: \"{}\", broad: \"{}\", gps: {} }}",
            precise, broad, gps_struct
        );
        let wallpaper_struct = format!(
            "&Wallpaper {{ file: \"{}\", date: \"{}\", location: {} }},\n",
            wallpaper.file, wallpaper.date, location_struct
        );

        wallpapers.push_str(&wallpaper_struct);
    }

    wallpapers.push_str("];\n");

    // Write string to file
    let mut output_wallpapers_path: PathBuf = CRATE_PATH.into();
    output_wallpapers_path.push("src/wallpapers.rs");
    let mut output_file =
        std::fs::File::create(&output_wallpapers_path).expect("file already exists");
    output_file
        .write_all(wallpapers.as_bytes())
        .expect("write dictionary to file");
}

// TODO: use something like https://github.com/oxidecomputer/typify

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Work {
    name: String,
    position: String,
    start_date: String,
    end_date: Option<String>,
    logo: Logo,
    description: String,
    highlights: Vec<String>,
    technologies: Vec<String>,
    link: Option<Link>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Project {
    name: String,
    description: String,
    start_date: String,
    end_date: Option<String>,
    presentation: Vec<String>,
    highlights: Vec<String>,
    keywords: Vec<String>,
    link: Option<Link>,
    logo: Option<Logo>,
    image: Option<Image>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Image {
    file: String,
    // TODO: use enum directly instead of string
    // position: ImagePosition,
    position: String,
}

// #[derive(Debug, Deserialize)]
// TODO: possible implementation: https://www.reddit.com/r/rust/comments/10bab4v/serdejson_how_to_deserialize_and_serialize_an/
// enum ImagePosition {
//     Left,
//     Right,
// }

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Link {
    uri: String,
    label: String,
    not_available: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Logo {
    file: String,
    transparent_background: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Education {
    institution: String,
    study_type: String,
    area: String,
    start_date: String,
    end_date: Option<String>,
    logo: Option<Logo>,
    courses: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Wallpaper {
    filename: String,
    date: String,
    gps: Gps,
    location: Location,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Gps {
    latitude: f32,
    longitude: f32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Location {
    precise: String,
    broad: String,
}

fn vec_strings(vec: &Vec<String>) -> String {
    let mut string = String::new();
    string.push('[');
    for el in vec {
        string.push_str(&format!("{:?},", el));
    }
    string.push(']');
    string
}

fn resume(dest: &std::path::Path, source: std::fs::File) {
    let reader = std::io::BufReader::new(source);
    let data: serde_json::Value = serde_json::from_reader(reader).expect("Failed to parse JSON");
    let mut final_ser = String::new();

    // Work
    let work_json = &data["work"];
    let work_data: Vec<Work> =
        serde_json::from_value(work_json.clone()).expect("Failed to parse work");
    let mut work_str = String::new();
    work_str.push('[');
    for work in &work_data {
        work_str.push_str("Work {");
        work_str.push_str(&format!("name: {:?},", work.name));
        work_str.push_str(&format!("position: {:?},", work.position));
        work_str.push_str(&format!("start_date: {:?},", work.start_date));

        work_str.push_str(&format!(
            "end_date: {},",
            match &work.end_date {
                Some(end) => format!("Some({:?})", end),
                None => "None".into(),
            }
        ));

        // Optional struct
        // TODO: refacto
        match &work.link {
            // TODO: enum with 2 variants for unavailable link
            Some(link) => match link.not_available {
                true => work_str.push_str(&format!(
                    "link: Some(ResumeLink {{ uri: {:?}, label: \"Not available\", not_available: true }}),",
                    link.uri
                )),
                false => work_str.push_str(&format!(
                    "link: Some(ResumeLink {{ uri: {:?}, label: {:?}, not_available: false }}),",
                    link.uri, link.label
                )),
            },
            None => work_str.push_str("link: None,"),
        }

        // Struct
        work_str.push_str(&format!(
            "logo: Logo {{ file: {:?}, transparent_background: {} }},",
            work.logo.file, work.logo.transparent_background
        ));

        work_str.push_str(&format!("description: {:?},", work.description));

        // Vector
        let mut highlights = String::new();
        highlights.push('[');
        for high in &work.highlights {
            highlights.push_str(&format!("{:?},", high));
        }
        highlights.push(']');
        work_str.push_str(&format!("highlights: &{},", highlights));

        // Vector
        let mut technologies = String::new();
        technologies.push('[');
        for tech in &work.technologies {
            technologies.push_str(&format!("{:?},", tech));
        }
        technologies.push(']');
        work_str.push_str(&format!("technologies: &{},", technologies));

        work_str.push_str("},\n");
    }
    work_str.push(']');
    let work_ser = format!(
        "pub const WORK: [Work; {}] = \n{};\n",
        work_data.len(),
        work_str
    );
    final_ser.push_str(&work_ser);

    // Projects
    let projects_json = &data["projects"];
    let projects_data: Vec<Project> =
        serde_json::from_value(projects_json.clone()).expect("Failed to parse projects");
    let mut projects_str = String::new();
    projects_str.push('[');
    for project in &projects_data {
        projects_str.push_str("Project {");
        projects_str.push_str(&format!("name: {:?},", project.name));
        projects_str.push_str(&format!("description: {:?},", project.description));
        projects_str.push_str(&format!("start_date: {:?},", project.start_date));

        projects_str.push_str(&format!(
            "end_date: {},",
            match &project.end_date {
                Some(end) => format!("Some({:?})", end),
                None => "None".into(),
            }
        ));

        // Optional struct
        match &project.link {
            // TODO: enum with 2 variants for unavailable link
            Some(link) => match link.not_available {
                true => projects_str.push_str(&format!(
                    "link: Some(ResumeLink {{ uri: {:?}, label: \"Not available\", not_available: true }}),",
                    link.uri
                )),
                false => projects_str.push_str(&format!(
                    "link: Some(ResumeLink {{ uri: {:?}, label: {:?}, not_available: false }}),",
                    link.uri, link.label
                )),
            },
            None => projects_str.push_str("link: None,"),
        }

        // Vector
        projects_str.push_str(&format!(
            "presentation: &{},",
            vec_strings(&project.presentation)
        ));

        // Vector
        projects_str.push_str(&format!(
            "highlights : &{},",
            vec_strings(&project.highlights)
        ));

        // Vector
        projects_str.push_str(&format!("keywords: &{},", vec_strings(&project.keywords)));

        // Optional struct
        match &project.logo {
            Some(logo) => projects_str.push_str(&format!(
                "logo: Some(Logo {{ file: {:?}, transparent_background: {} }}),",
                logo.file, logo.transparent_background
            )),
            None => projects_str.push_str("logo: None,"),
        }

        // Optional struct
        match &project.image {
            Some(image) => projects_str.push_str(&format!(
                "image: Some(Image {{ file: {:?}, position: {:?} }}),",
                image.file, image.position
            )),
            None => projects_str.push_str("image: None,"),
        }

        projects_str.push_str("},\n");
    }
    projects_str.push(']');
    let projects_ser = format!(
        "pub const PROJECTS: [Project; {}] = \n{};\n",
        projects_data.len(),
        projects_str
    );
    final_ser.push_str(&projects_ser);

    // Education
    let education_json = &data["education"];
    let education_data: Vec<Education> =
        serde_json::from_value(education_json.clone()).expect("Failed to parse education");
    let mut education_str = String::new();
    education_str.push('[');
    for education in &education_data {
        education_str.push_str("Education {");
        education_str.push_str(&format!("institution: {:?},", education.institution));
        education_str.push_str(&format!("area: {:?},", education.area));
        education_str.push_str(&format!("study_type: {:?},", education.study_type));
        education_str.push_str(&format!("start_date: {:?},", education.start_date));
        education_str.push_str(&format!(
            "end_date: {},",
            match &education.end_date {
                Some(end) => format!("Some({:?})", end),
                None => "None".into(),
            }
        ));

        // Optional struct
        match &education.logo {
            Some(logo) => education_str.push_str(&format!(
                "logo: Some(Logo {{ file: {:?}, transparent_background: {} }}),",
                logo.file, logo.transparent_background
            )),
            None => education_str.push_str("logo: None,"),
        }

        // Vector
        let mut courses = String::new();
        courses.push('[');
        for course in &education.courses {
            courses.push_str(&format!("{:?},", course));
        }
        courses.push(']');
        education_str.push_str(&format!("courses: &{},", courses));
        education_str.push_str("},\n");
    }
    education_str.push(']');
    let education_ser = format!(
        "pub const EDUCATION: [Education; {}] = \n{};\n",
        education_data.len(),
        education_str
    );
    final_ser.push_str(&education_ser);

    std::fs::write(dest, final_ser).unwrap();
}

fn wallpapers(dest: &std::path::Path, source: std::fs::File) {
    let reader = std::io::BufReader::new(source);
    let wallpapers_data: Vec<Wallpaper> =
        serde_json::from_reader(reader).expect("Failed to parse JSON");
    let mut final_ser = String::new();

    let mut wallpapers_str = String::new();
    wallpapers_str.push('[');

    for wallpaper in &wallpapers_data {
        wallpapers_str.push_str("Wallpaper {");
        wallpapers_str.push_str(&format!("filename: {:?},", wallpaper.filename));
        wallpapers_str.push_str(&format!("date: {:?},", wallpaper.date));
        wallpapers_str.push_str(&format!(
            "gps: Gps {{ latitude: {}, longitude: {} }},",
            wallpaper.gps.latitude, wallpaper.gps.longitude
        ));
        wallpapers_str.push_str(&format!(
            "location: Location {{ precise: {:?}, broad: {:?} }},",
            wallpaper.location.precise, wallpaper.location.broad
        ));
        wallpapers_str.push_str("},\n");
    }

    wallpapers_str.push(']');
    let wallpapers_ser = format!(
        "pub const WALLPAPERS: [Wallpaper; {}] = \n{};\n",
        wallpapers_data.len(),
        wallpapers_str
    );
    final_ser.push_str(&wallpapers_ser);

    std::fs::write(dest, final_ser).unwrap();
}

fn main() {
    println!("cargo::rerun-if-changed=resume.json");
    println!("cargo::rerun-if-changed=wallpapers.json");

    let out_dir = std::env::var_os("OUT_DIR").unwrap();

    let resume_dest = std::path::Path::new(&out_dir).join("resume.rs");
    let resume_source = std::fs::File::open("resume.json").expect("Failed to open file");
    resume(&resume_dest, resume_source);

    let wallpapers_dest = std::path::Path::new(&out_dir).join("wallpapers.rs");
    match std::fs::File::open("wallpapers.json") {
        Ok(source) => wallpapers(&wallpapers_dest, source),
        Err(_) => {
            std::fs::write(wallpapers_dest, "pub const WALLPAPERS: [Wallpaper; 0] = [];").unwrap();
            println!("cargo::warning=skipping wallpapers, file not found");
        }
    }
}

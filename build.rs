fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=tailwind.config.cjs");
    println!("cargo:rerun-if-changed=templates/");

    let result = std::process::Command::new("tailwindcss")
        .arg("-i")
        .arg("./css/tailwind.css")
        .arg("-o")
        .arg("./public/style.css")
        .output()
        .unwrap();

    println!("{:?}", result);

    if !result.status.success() {
        panic!("Failed to run tailwindcss")
    }

    Ok(())
}

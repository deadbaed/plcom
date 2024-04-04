use clap::Parser;
use gen_wallpapers::MetadataList;
use std::fs::read_dir;
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    folder: PathBuf,
}

fn main() {
    let cli = Cli::parse();
    let dir = match read_dir(cli.folder) {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("failed to read folder: {e}");
            return;
        }
    };

    let metadata = MetadataList::process_folder(dir, true);

    let json = match metadata.to_json() {
        Ok(json) => json,
        Err(e) => {
            eprintln!("failed to serialize json: {e}");
            return;
        }
    };
    println!("{json}");
}

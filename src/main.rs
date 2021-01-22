use std::path::Path;
use std::env;
use std::fs;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const OS: &'static str = env::consts::OS;


struct Launcher {
    form: String,
    path: String
}

fn main() {
    println!("Welcome to the kami-installer v{}.", VERSION);
    let mut launcher_path: Vec<Launcher> = Vec::new();
    let mut profiles: Vec<String> = Vec::new();

    match OS {
        "macos" => {
            if Path::new("/Applications/MultiMC.app").exists() {
                let files = fs::read_dir("/Applications/MultiMC.app/Contents/MacOS/instances").unwrap();
                for entry in files {
                    let entry = entry.unwrap();
                    profiles.push(format!("{:?}", entry.path().file_name().ok_or("No profiles found!")));
                }
            }
            if Path::new("/Applications/Minecraft.app").exists() {
                //
            }
        }
        _ => panic!("Your operating system {:?} is not supported right now.", OS)
    }

    println!("Found {} profiles.", profiles.len());
    for i in profiles {
        println!("{}", i);
    }
}

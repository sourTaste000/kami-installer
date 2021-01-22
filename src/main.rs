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
    let mut profiles: Vec<String> = Vec::new();
    let mut sanitized_profiles: Vec<String> = Vec::new();

    match OS {
        "macos" => {
            if Path::new("/Applications/MultiMC.app/Contents/MacOS/instances").exists() {
                for entry in fs::read_dir("/Applications/MultiMC.app/Contents/MacOS/instances").unwrap() {
                    let entry = entry.unwrap();
                    profiles.push(format!("{:?}", entry.path().file_name().ok_or("Not found!")));
                }
                for instances in &profiles {
                    let sanitized = instances.replace("Ok(\"", "").replace("\")", "");
                    if sanitized == "instgroups.json" || sanitized == "_MMC_TEMP"{
                        continue
                    }
                    sanitized_profiles.push(sanitized)
                }
            }
            // if Path::new("/Applications/Minecraft.app").exists() {
            //
            // }
        }

        "windows" => {
            if Path::new("C:\\Program Files\\MultiMC\\instances").exists() {
                for entry in fs::read_dir("C:\\Program Files\\MultiMC\\instances").unwrap() {
                    let entry = entry.unwrap();
                    profiles.push(format!("{:?}", entry.path().file_name().ok_or("No profiles found!")));
                }
            }
        }

        _ => panic!("Your operating system {:?} is not supported right now.", OS)
    }

    println!("Found {} profiles.", sanitized_profiles.len());
    for versions in sanitized_profiles {
        println!("{}", versions);
    }
}

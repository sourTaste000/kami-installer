use std::path::Path;
use std::env;
use std::fs;
use download_rs::sync_download::Download;
use std::error::Error;
use std::process::Command;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const OS: &'static str = env::consts::OS;

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
    for i in 0..sanitized_profiles.len() {
        println!("{}. {}", i, sanitized_profiles[i])
    }

    let mut selected = String::new();
    println!("Choose the profile you want to install: ");
    let _ = std::io::stdin().read_line(&mut selected).unwrap();
    let chosen_profile = sanitized_profiles[selected.trim().parse::<usize>().unwrap()].to_owned();

    let version = match fs::read_to_string(format!("/Applications/MultiMC.app/Contents/MacOS/instances/{}/mmc-pack.json", chosen_profile)) {
        Ok(contents) => contents,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };


    // I got lazy here
    if version.contains("1.12.2") {
        println!("1.12.2")
    } else if version.contains("1.16.4") {
        let download = match OS {
            "macos" => {
                Command::new("curl")
                    .arg("-LO")
                    .arg("https://github.com/zeroeightysix/KAMI/releases/download/1.16.4-dec/KAMI-1.16.4-dec.jar")
                    .output()
                    .expect("Failed to execute!");
            }
            _ => panic!("not supported")
        };
        let mv = match OS {
            "macos" => {
                Command::new("mv")
                    .arg("./KAMI-1.16.4-dec.jar")
                    .arg(format!("/Applications/MultiMC.app/Contents/MacOS/instances/{}/.minecraft/mods/", chosen_profile))
                    .output()
                    .expect("Failed to execute")
            }
            _ => panic!("Not supported")
        };
        println!("done!")
    } else {
        panic!("Version not supported!")
    }
}

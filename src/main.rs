use std::path::Path;
use std::env;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const OS: &'static str = env::consts::OS;


struct Launcher {
    form: String,
    path: String
}

fn main() {
    println!("Welcome to the kami-installer v{}.", VERSION);
    let mut launcher_path: Vec<Launcher> = Vec::new();

    if OS == "macos" {
        println!("macOS detected");
        if Path::new("/Applications/MultiMC.app").exists() {
            launcher_path.push(Launcher {
                form: "multimc".to_string(),
                path: "/Applications/MultiMC.app".to_string()
            })
        }

        if Path::new("/Applications/Minecraft.app").exists() {
            launcher_path.push( Launcher {
                form: "vanilla".to_string(),
                path: "/Applications/Minecraft.app".to_string()
            })
        }
    } else if OS == "linux" {
        println!("linux detected");
    } else if OS == "windows" {
        println!("windows detected");
        if Path::new("C:\\Program Files\\MultiMC").exists() {
            launcher_path.push(Launcher {
                form: "multimc".to_string(),
                path: "C:\\Program Files\\MultiMC".to_string()
            })
        }

        if Path::new("C:\\Program Files\\Minecraft").exists() {
            launcher_path.push(Launcher {
                form: "vanilla".to_string(),
                path: "C:\\Program Files\\Minecraft".to_string()
            })
        }
    }

    println!("Found {} instances.", launcher_path.len());
    for i in launcher_path {
        print!("Launcher type: {}, Launcher path: {}\n", i.form, i.path);
    }
}

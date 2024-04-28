use std::fs::{create_dir_all, remove_dir_all, File};
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

use colored::Colorize;
use reqwest;

fn main() {
    let data = fetch_web_page_content("file:///C:/Users/thibault.monnier/Documents/Learn-Rust/txt-file-reader/cargo_toml.html".to_string());
    //let data_txt_contents = extract_file_contents(PathBuf::from("src").join("data.txt"));

    let generated_directory_path = PathBuf::from("generated");
    generate_directory(&generated_directory_path);

    // Create generated.rs file with the data
    let generated_file_path = generated_directory_path.join("generated.rs");
    let _generated_src_file = File::create(&generated_file_path)
        .unwrap()
        .write(data.as_bytes())
        .expect("Unable to write data");

    // Create Cargo.toml file
    let generated_cargo_toml_file_path = generated_directory_path.join("Cargo.toml");
    let cargo_toml_txt_contents =
        fetch_web_page_content();
    let _generate_cargo_toml_file = File::create(&generated_cargo_toml_file_path)
        .unwrap()
        .write(cargo_toml_txt_contents.as_bytes())
        .expect("Unable to write data");

    // Compile and run the generated.rs file
    run_command(&format!(
        "cargo run --manifest-path {}",
        generated_cargo_toml_file_path.display()
    ));
    println!("{}", "File compiled successfully!".bright_cyan());
}

fn generate_directory(path: &PathBuf) {
    // Can't do path.exists() if path is a string
    if path.exists() {
        remove_dir_all(path).expect("Failed to remove the existing directory")
    }

    create_dir_all(path).expect("Unable to create directory");
    println!(
        "{}",
        &format!("Directory created at: {}", path.display()).bright_cyan()
    );
}

/*fn extract_file_contents(path: PathBuf) -> String {
    let mut data_file: File = match File::open(path) {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let seperator = "----------------------------------------------------------------------";
    let mut data_file_contents = String::new();
    match data_file.read_to_string(&mut data_file_contents) {
        Ok(_) => println!(
            "\nFile content:\n{}\n{}{}",
            seperator,
            data_file_contents.to_string().bright_black(),
            seperator
        ),
        Err(error) => eprintln!("Problem reading the file: {:?}", error),
    };

    return data_file_contents;
}*/

fn run_command(command: &str) {
    // The command system is different for Windows and Unix-based systems
    #[cfg(target_os = "windows")]
    let output = Command::new("cmd").args(&["/C", command]).output();

    #[cfg(not(target_os = "windows"))]
    let output = Command::new("sh").arg("-c").arg(command).output();

    match output {
        Ok(_) => {
            println!(
                "{}: {}",
                "Command executed successfully!".bright_cyan(),
                command.yellow()
            );
        }
        Err(e) => {
            eprintln!("{}{}", "Failed to run command: ".red(), e.to_string().red());
        }
    }
}

fn fetch_web_page_content(url: String) -> String {
    return reqwest::blocking::get(url).unwrap().text().unwrap()
}

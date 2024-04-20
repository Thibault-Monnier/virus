fn main() {
    for _ in 0..1 {
        match std::process::Command::new("notepad.exe").spawn() {
            Ok(e) => println!("{:?}", e),
            Err(e) => println!("{}", e),
        }
    }
}

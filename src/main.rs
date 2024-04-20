fn main() {
    match std::process::Command::new("notepad.exe").spawn() {
        Ok(e) => println!("{:?}", e),
        Err(e) => println!("{}", e),
    }
    match std::process::Command::new("renamed-again.exe").spawn() {
        Ok(e) => println!("{:?}", e),
        Err(e) => println!("{}", e),
    }
    println!("a b c d e f g h");
}

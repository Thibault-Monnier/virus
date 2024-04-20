fn main() {
    let user_wants_to_open_notepad = true;
    for _ in 0..3000 {
        if user_wants_to_open_notepad {
            match std::process::Command::new("notepad.exe").spawn() {
                Ok(e) => println!("{:?}", e),
                Err(e) => println!("{}", e),
            }
        }
    }
}

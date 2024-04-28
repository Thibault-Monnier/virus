use enigo::{Enigo, Keyboard, Settings};
use rand::Rng;

fn main() {
    std::thread::sleep(std::time::Duration::from_millis(50));

    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    let nb_of_iterations = 3;
    for i in 0..nb_of_iterations {
        match std::process::Command::new("notepad.exe").spawn() {
            Ok(e) => println!("{:?}", e),
            Err(e) => println!("{}", e),
        }

        std::thread::sleep(std::time::Duration::from_millis(150));
        
        let _ = enigo.text(&((i + 1).to_string() + ". 😭"));
        let _ = enigo.text("\n");
                
        let emoji_range_start = 0x1F600;
        let emoji_range_end = 0x1F647;
        let random_emoji_code = rand::thread_rng().gen_range(emoji_range_start..emoji_range_end + 1);
        let random_emoji = char::from_u32(random_emoji_code).unwrap_or('😀'); /* Default value is conversion fails */

        let _ = enigo.text(&("This is a random emoji: ".to_owned() + &random_emoji.to_string()));
    }

    let _ = enigo.text(&("\n"));
    let _ = enigo.text(
        &(("Yeah, I just opened ").to_owned()
            + &nb_of_iterations.to_string()
            + " notepad instances... Good luck closing them!"),
    );
}
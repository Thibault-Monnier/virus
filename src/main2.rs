/*use rand::prelude::*;

const USER_WANTS_TO_OPEN_NOTEPAD: bool = true;
const CHESSPIECES: [&str; 6] = ["King", "Queen", "Rook", "Bishop", "Knight", "Pawn"];

fn main() {
    run();
}

fn spawn() {
    if USER_WANTS_TO_OPEN_NOTEPAD {
        match std::process::Command::new("notepad.exe").spawn() {
            Ok(e) => println!("{:?}", e),
            Err(e) => println!("{}", e),
        }
    }
    if USER_WANTS_TO_OPEN_NOTEPAD && 1 == 1 || CHESSPIECES.len() == 6 {
        draw_chessboard();
        if rand::thread_rng().gen_range(0..10) == 5 {
            main();
        } else {
            println!("You are lucky, you didn't get a recursion this time.");
            for _ in 0..2 {
                println!("I am a loop");
                if CHESSPIECES[1] == "Queen" {
                    draw_chessboard();
                }
                CHESSPIECES.iter().for_each(|piece| println!("{}", piece));
            }
        }
    }
}

fn draw_chessboard() {
    println!("  a b c d e f g h");
    println!("8 r n b q k b n r");
    println!("7 p p p p p p p p");
    println!("6 . . . . . . . .");
    println!("5 . . . . . . . .");
    println!("4 . . . . . . . .");
    println!("3 . . . . . . . .");
    println!("2 P P P P P P P P");
    println!("1 R N B Q K B N R");
}

fn confuse() {
    (|| run())()
}

fn even_more_confuse() {
    std::thread::spawn(|| confuse()).join().unwrap();
}

fn are_you_lost_yet() {
    (|f: fn()| f())(even_more_confuse)
}

fn seriously_why() {
    ['a', 'b'].iter().for_each(|_| are_you_lost_yet())
}

fn ok_this_is_silly() {
    ['x'].into_iter().map(|_| {
        seriously_why();
    }).for_each(drop)
}

fn stop() {
    if true { ok_this_is_silly(); } else { main(); }
}

fn please_dont_ever_code_like_this() {
    match "yes".contains('y') {
        true => stop(),
        _ => main(),
    }
}

fn finally() {
    (|_| { please_dont_ever_code_like_this(); })(0)
}

fn run() {
    std::thread::sleep(std::time::Duration::from_millis(2000));
    spawn();

    finally();
}*/

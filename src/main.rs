use colored::Colorize;
use magic_8::Config;
use rand::Rng;
use std::env;
use std::process;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });

    let num: usize = rand::thread_rng().gen_range(0..=20);

    let responses: Vec<String> = vec![
        String::from("IT IS CERTAIN"),
        String::from("IT IS DECIDEDLY SO"),
        String::from("WITHOUT A DOUBT"),
        String::from("YES, DEFINITELY"),
        String::from("YOU MAY RELY ON IT"),
        String::from("AS I SEE IT, YES"),
        String::from("MOST LIKELY"),
        String::from("OUTLOOK GOOD"),
        String::from("YES"),
        String::from("SIGNS POINT TO YES"),
        String::from("REPLY HAZY, TRY AGAIN"),
        String::from("ASK AGAIN LATER"),
        String::from("BETTER NOT TELL YOU NOW"),
        String::from("CANNOT PREDICT NOW"),
        String::from("CONCENTRATE AND ASK AGAIN"),
        String::from("DON'T COUNT ON IT"),
        String::from("MY REPLY IS NO"),
        String::from("MY SOURCES SAY NO"),
        String::from("OUTLOOK NOT SO GOOD"),
        String::from("VERY DOUBTFUL"),
        String::from("ONLY SITH DEAL IN ABSOLUTES"),
    ];

    let color = match num {
        0..=9 => "green",
        10..=14 => "yellow",
        _ => "red",
    };

    print!("\x1B[2J\x1B[1;1H");
    println!();
    println!("=========================================================================");
    println!();
    println!("oooo     oooo      o        ooooooo8  ooooo   oooooooo8        ooooooo   ");
    println!(" 8888o   888      888     o888    88   888  o888     88      o888   888o ");
    println!(" 88 888o8 88     8  88    888    oooo  888  888               888888888  ");
    println!(" 88  888  88    8oooo88   888o    88   888  888o     oo      888o   o888 ");
    println!("o88o  8  o88o o88o  o888o  888ooo888  o888o  888oooo88         88ooo88   ");
    println!();
    println!("=========================================================================");
    println!();
    println!("You asked the ball: {}", config.question.color("magenta"));
    println!();
    println!(
        "The ball has determined: {}",
        responses[num].color(color).blink()
    );
    println!();
}

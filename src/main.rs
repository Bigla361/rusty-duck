use std::{thread, time};
use rand::Rng;

const EYES: [char; 4] = ['o', 'o', 'o', '~'];
const BEAKS: [char; 5] = ['<', '<', '<', '<', '>',];
const WATER: char = '~';
const DUCK: &str = "ag     ̥ \nap    /\\\nap   /y__p\\\na  rny(e )___\na   ( ._> /\ncfy`---'cb";
const COLOURS: [char; 5] = ['r', 'y', 'c', 'p', 'g'];

fn main() {
    let mut water_behind: u8 = 4;

    loop {
        render(water_behind);

        // Increment water counter
        if water_behind != 26 {
            water_behind += 1
        }

        thread::sleep(time::Duration::from_millis(200));
    }
}

fn render(water_behind: u8) {
    // Get eyes and beak
    let eye = rand::thread_rng().gen_range(1..EYES.len());
    let eye = &EYES[eye].to_string();

    let beak = rand::thread_rng().gen_range(1..BEAKS.len());
    let beak = &BEAKS[beak].to_string();

    // Create a duck
    let mut current_duck = DUCK.replace("b", &multiply_chars(WATER, water_behind));
    current_duck = current_duck.replace("a", &multiply_chars(' ', 26-water_behind));
    current_duck = current_duck.replace("f", &multiply_chars(WATER, 26-water_behind+4));
    current_duck = current_duck.replace("e", eye);
    current_duck = current_duck.replace("n", beak);

    for colour in COLOURS {
        current_duck = current_duck.replace(&colour.to_string(), match colour {
            'r' => "\x1b[31m",
            'y' => "\x1b[93m",
            'c' => "\x1b[36m",
            'p' => "\x1b[35m",
            'g' => "\x1b[32m",
            _ => ""
        })
    }

    println!("\x1B[2J\x1B[1;1H{current_duck}");
}

fn multiply_chars(c: char, amount: u8) -> String {
    vec![c.to_string(); amount.into()].join("")
}
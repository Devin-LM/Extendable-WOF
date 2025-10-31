mod jsonloader;
use std::io;

fn main() {
    let filename: String = String::from("datatest/test.json");
    let data = jsonloader::load_game_data(&filename);
    println!("{:?}", data);

    let mut input = String::new();
    while input.trim() != "e" {

        input.clear();
        io::stdin().read_line(&mut input).expect("Could not read user input");

        match input.trim() {
            "r" => println!("{:?}", jsonloader::load_game_data(&filename)),
            _ => println!("Bad input"),
        }
    }
}

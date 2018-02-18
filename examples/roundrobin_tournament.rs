extern crate gamer;

use std::io::{self, BufRead};

use gamer::matchers::simple::RoundRobin;

fn main() {
    let stdin = io::stdin();

    // read names
    let mut players: Vec<String> = Vec::new();
    for line in stdin.lock().lines() {
        let ln = line.unwrap();
        players.push(ln.trim().to_string());
    }

    // print games
    for m in RoundRobin::new(players) {
        println!("{} vs {}", m[0], m[1]);
    }
}

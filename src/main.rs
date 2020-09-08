use std::fs;

mod malaphor;

fn main() {
    let malaphor = malaphor::Malaphor::new(&fs::read_to_string("./src/sentences.txt").unwrap());

    let result = malaphor.generate();

    println!("\nrandom malaphor: {}", result);
}

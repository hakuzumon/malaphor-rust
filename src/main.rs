mod malaphor;

fn main() {
    let malaphor = malaphor::Malaphor::new("./src/sentences.txt");

    let result = malaphor.generate();

    println!("\nrandom malaphor: {}", result);
}

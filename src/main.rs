mod malaphor;

fn main() {
    let contents = include_str!("sentences.txt");
    let malaphor = malaphor::Malaphor::new(contents);

    let result = malaphor.generate();

    println!("\nrandom malaphor: {}", result);
}

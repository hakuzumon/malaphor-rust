mod malaphor {
    use std::{fmt, fs};
    use std::collections::HashMap;
    use std::fmt::Formatter;

    use rand::distributions::Uniform;
    use rand::prelude::*;

    #[derive(Debug)]
    struct SentencePart {
        first_word_lowercase: String,
        part_index: usize,
        part: String
    }

    impl SentencePart {
        fn parse(part: &str, index: usize) -> SentencePart {
            let val = part.trim();

            let first_space = val.find(' ');

            let first_word = match first_space {
                Some(i) => &val[..i],
                None => &val[..]
            };

            SentencePart {
                part: val.to_string(),
                first_word_lowercase: first_word.to_lowercase().to_string(),
                part_index: index
            }
        }
    }

    impl fmt::Display for SentencePart {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.part)
        }
    }

    #[derive(Debug)]
    pub struct Sentence {
        parts: Vec<SentencePart>
    }

    impl Sentence {
        fn parse(line: &str) -> Sentence {

            let parts: Vec<SentencePart> = line.split(", ")
                .enumerate()
                .map(|(i, s)| SentencePart::parse(s, i))
                .collect();

            Sentence {
                parts
            }
        }
    }

    impl fmt::Display for Sentence {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            let parts: Vec<String> = self.parts.iter().map(|x| x.part.to_string()).collect();
            write!(f, "{}", parts.join(", "))
        }
    }

    pub struct Malaphor {
        data: Vec<Sentence>,

        rng: ThreadRng
    }

    impl Malaphor {

        pub fn init(file_path: &str) -> Malaphor {
            let data = Malaphor::load_aphorisms(file_path);
            let rng = thread_rng();

            Malaphor {
                data,
                rng
            }
        }

        fn load_aphorisms(file_path: &str) -> Vec<Sentence> {
            let file = fs::read_to_string(file_path).unwrap();

            let mut sentences_by_connecting_word: HashMap<String, Vec<Sentence>> = HashMap::new();

            let sentences: Vec<Sentence> = file.split("\n").map(Sentence::parse).collect();

            sentences.into_iter()
                .filter(|t| t.parts.len() == 2)
                .for_each(|sentence| {
                    let key = sentence.parts[1].first_word_lowercase.to_string();
                    sentences_by_connecting_word.entry(key)
                        .or_insert(Vec::new())
                        .push(sentence);
                });

            sentences_by_connecting_word.into_iter()
                // only take sentences which have at least one "switchable partner"
                .filter(|(_, s)| s.len() > 1)
                .flat_map(|(_, ss)| ss)
                .collect()
        }

        pub fn random_sentence(&self) -> &Sentence {
            let mut rng = self.rng;
            let sample_index = rng.sample(Uniform::new(0, self.data.len()));
            &self.data[sample_index]
        }
    }
}

fn main() {
    let malaphor = malaphor::Malaphor::init("./src/sentences.txt");

    println!("random: {}", malaphor.random_sentence());
}

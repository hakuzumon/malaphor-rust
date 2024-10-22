use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;

use rand::prelude::*;

#[derive(Debug, PartialEq)]
struct SentencePart {
    first_word_lowercase: String,
    part_index: usize,
    part: String,
}

impl SentencePart {
    fn parse(part: &str, index: usize) -> SentencePart {
        let val = part.trim();

        let first_word = val.split(' ').next().unwrap_or(val);

        SentencePart {
            part: val.to_owned(),
            first_word_lowercase: first_word.to_lowercase(),
            part_index: index,
        }
    }
}

impl fmt::Display for SentencePart {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&self.part)
    }
}

#[derive(Debug, PartialEq)]
pub struct Sentence {
    parts: Vec<SentencePart>,
}

impl Sentence {
    fn parse(line: &str) -> Sentence {
        Sentence {
            parts: line
                .split(", ")
                .enumerate()
                .map(|(i, s)| SentencePart::parse(s, i))
                .collect(),
        }
    }
}

impl fmt::Display for Sentence {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", itertools::join(self.parts.iter(), ", "))
    }
}

pub struct Malaphor {
    data: Vec<Sentence>,
}

impl Malaphor {
    pub fn new(file_contents: &str) -> Malaphor {
        Malaphor {
            data: Malaphor::load_aphorisms(file_contents),
        }
    }

    fn load_aphorisms(file_contents: &str) -> Vec<Sentence> {
        let mut sentences_by_connecting_word: HashMap<String, Vec<Sentence>> = HashMap::new();

        let sentences: Vec<_> = file_contents.lines().map(Sentence::parse).collect();

        sentences
            .into_iter()
            .filter(|t| t.parts.len() == 2)
            .for_each(|sentence| {
                let key = sentence.parts[1].first_word_lowercase.to_string();
                sentences_by_connecting_word
                    .entry(key)
                    .or_insert_with(Vec::new)
                    .push(sentence);
            });

        sentences_by_connecting_word
            .into_iter()
            // only take sentences which have at least one "switchable partner"
            .filter(|(_, s)| s.len() > 1)
            .flat_map(|(_, ss)| ss)
            .collect()
    }

    pub fn find_matches(&self, sentence: &Sentence) -> Vec<&Sentence> {
        let good_matches = self.find_good_matches(sentence);
        let bad_match_probability = self.get_bad_match_probability(good_matches.len());

        if thread_rng().gen_bool(bad_match_probability) {
            self.find_bad_matches(sentence)
        } else {
            good_matches
        }
    }

    fn find_good_matches(&self, sentence: &Sentence) -> Vec<&Sentence> {
        self.data
            .iter()
            .filter(|&s| {
                s != sentence
                    && s.parts[1].first_word_lowercase == sentence.parts[1].first_word_lowercase
            })
            .collect()
    }

    fn find_bad_matches(&self, sentence: &Sentence) -> Vec<&Sentence> {
        self.data.iter().filter(|&s| s != sentence).collect()
    }

    fn get_bad_match_probability(&self, option_count: usize) -> f64 {
        (match option_count {
            1 => 95,
            2 => 90,
            3 => 80,
            4..=6 => 70,
            7..=9 => 60,
            10..=14 => 30,
            _ => 15,
        } as f64)
            / 100.0
    }

    fn combine_in_any_order(&self, s1: &Sentence, s2: &Sentence) -> String {
        if thread_rng().gen() {
            self.combine(s1, s2)
        } else {
            self.combine(s2, s1)
        }
    }

    fn combine(&self, begin: &Sentence, end: &Sentence) -> String {
        format!("{}, {}", begin.parts[0].part, end.parts[1].part)
    }

    pub fn generate(&self) -> String {
        let rng = &mut thread_rng();
        let s1 = self.data.choose(rng).unwrap();
        let matches = self.find_matches(s1);
        let s2 = *matches.choose(rng).unwrap();

        self.combine_in_any_order(s1, s2)
    }
}

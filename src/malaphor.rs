use std::{fmt, fs};
use std::collections::HashMap;
use std::fmt::Formatter;

use rand::distributions::Uniform;
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
        write!(f, "{}", self.part)
    }
}

#[derive(Debug, PartialEq)]
pub struct Sentence {
    parts: Vec<SentencePart>
}

impl Sentence {
    fn parse(line: &str) -> Sentence {
        Sentence {
            parts: line.split(", ")
            .enumerate()
            .map(|(i, s)| SentencePart::parse(s, i))
            .collect()
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

    rng: ThreadRng,
}

impl Malaphor {
    pub fn new(file_path: &str) -> Malaphor {
        let data = Malaphor::load_aphorisms(file_path);
        let rng = thread_rng();

        Malaphor {
            data,
            rng,
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
        let choices = &self.data;
        let mut rng = self.rng;
        let sample_index = rng.sample(Uniform::new(0, choices.len()));
        &choices[sample_index]
    }

    pub fn random<'a>(&self, choices: &Vec<&'a Sentence>) -> &'a Sentence {
        let mut rng = self.rng;
        let sample_index = rng.sample(Uniform::new(0, choices.len()));
        choices[sample_index]
    }

    pub fn find_matches(&self, sentence: &Sentence) -> Vec<&Sentence> {
        let good_matches = self.find_good_matches(sentence);
        let bad_match_percentage = self.get_bad_match_percentage(good_matches.len());
        let mut rng = self.rng;
        let d100 = rng.sample(Uniform::new(0, 100));

        if d100 <= bad_match_percentage {
            self.find_bad_matches(sentence)
        } else {
            good_matches
        }
    }

    fn find_good_matches(&self, sentence: &Sentence) -> Vec<&Sentence> {
        self.data.iter()
            .filter(|s| s != &sentence &&
                s.parts[1].first_word_lowercase == sentence.parts[1].first_word_lowercase)
            .collect()
    }

    fn find_bad_matches(&self, sentence: &Sentence) -> Vec<&Sentence> {
        self.data.iter()
            .filter(|s| s != &sentence)
            .collect()
    }

    fn get_bad_match_percentage(&self, option_count: usize) -> u8 {
        match option_count {
            1 => 95,
            2 => 90,
            3 => 80,
            4 | 5 | 6 => 70,
            7 | 8 | 9 => 60,
            10 | 11 | 12 | 13 | 14 => 30,
            _ => 15
        }
    }

    fn combine_in_any_order(&self, s1: &Sentence, s2: &Sentence) -> String {
        let mut rng = self.rng;
        if rng.gen_bool(0.5) {
            self.combine(s1, s2)
        } else {
            self.combine(s2, s1)
        }
    }

    fn combine(&self, begin: &Sentence, end: &Sentence) -> String {
        begin.parts[0].part.to_string() + ", " + end.parts[1].part.to_string().as_str()
    }

    pub fn generate(&self) -> String {
        let s1 = self.random_sentence();
        let matches = self.find_matches(&s1);
        let s2 = self.random(&matches);

        self.combine_in_any_order(s1, s2)
    }
}


use rand::{Rng, SeedableRng};
use chrono::prelude::*;
use leptos::logging::log;
use rand::seq::SliceRandom;
use crate::words::WORDS;


pub struct LetterGenerator {
   words: Vec<&'static str>,
   chars: Vec<char>,
   seq: usize,
}
const WORD_COUNT: usize = 15;

pub const MIN_WORD_SIZE: usize = 3;

impl LetterGenerator {

   // Generate a sequence of 25 words from the random list. This will be the daily challenge.
   pub fn new() -> LetterGenerator {
      let current_date = Utc::now();

      // Extract the date components (year, month, day) as integers
      let year = current_date.year() as u32;
      let month = current_date.month() as u32;
      let day = current_date.day() as u32;

      // Concatenate the date components to form the seed value
      let seed: [u32; 8] = [year, month, day, 0, 0, 0, 0, 0];
      let mut seed_bytes = [0u8; 32];
      for (i, &n) in seed.iter().enumerate() {
         seed_bytes[i * 4..(i + 1) * 4].copy_from_slice(&n.to_be_bytes());
      }

      // Initialize the PRNG with the seed value
      let mut rng = rand::rngs::StdRng::from_seed(seed_bytes);

      let mut words = vec![];
      let mut i = 0;
      while i < WORD_COUNT {
         let w = WORDS[rng.gen_range(0..WORDS.len())];
         if w.len() < MIN_WORD_SIZE {
            continue
         }
         i += 1;
         words.push(w)
      }

      let mut chars = vec![];
      for word in &words {
         let mut seq: Vec<usize> = (0..word.len()).collect();
         seq.shuffle(&mut rng);
         while seq.len() > 0 {
            chars.push(word.chars().nth(seq.pop().unwrap()).unwrap())
         }
      }
      log!{"{:?}", words}
      log!{"{:?}", chars}
      LetterGenerator{
         words,
         chars,
         seq: 0,
      }
   }
}

impl Generator for LetterGenerator {
   fn next_letter (&mut self) -> Option<char>{
      if self.seq == self.chars.len() {
         return None
      }
      self.seq += 1;
      return Some(self.chars[self.seq - 1])
   }

   fn next_n_letters(&mut self, n: usize) -> Vec<char>{
     return self.chars[self.seq..std::cmp::min(self.seq + n, self.chars.len())].to_vec()
   }

   fn num_letters_left(&mut self) -> usize {
      return self.chars.len() - self.seq
   }

   fn calc_score(&self, calc_score: fn(&Vec<&str>) -> u32) -> u32 {
      return calc_score(&self.words)
   }
}


const TEST_LETTERS: &str = "CATDOGANDBUTSOLDCOUNTMATH";
pub struct TestGenerator {
   idx: usize
}

impl TestGenerator {
   pub fn new() -> TestGenerator {
      TestGenerator{
         idx: 0,
      }
   }
}
impl Generator for TestGenerator {


   fn next_letter(&mut self) -> Option<char>{
      self.idx += 1;
      return Some(TEST_LETTERS.chars().nth(self.idx - 1).unwrap())
   }

   fn next_n_letters(&mut self, n: usize) -> Vec<char>{
      let substring = &TEST_LETTERS[self.idx..std::cmp::min(self.idx + n, TEST_LETTERS.len())];
      substring.chars().collect()
   }

   fn num_letters_left(&mut self) -> usize {
      return TEST_LETTERS.len() - self.idx
   }

   fn calc_score(&self, calc_score: fn(&Vec<&str>) -> u32) -> u32 {
      return calc_score(&vec![TEST_LETTERS])
   }
}

pub trait Generator {
   fn next_letter(&mut self) -> Option<char>;

   fn next_n_letters(&mut self, n: usize) -> Vec<char>;

   fn num_letters_left(&mut self) -> usize;

   fn calc_score(&self, calc_score: fn(&Vec<&str>) -> u32) -> u32;
}

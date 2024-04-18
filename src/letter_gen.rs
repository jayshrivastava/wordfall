use rand::{Rng, SeedableRng};
use leptos::logging::log;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct LetterGenerator {
   words: Vec<String>,
   chars: Vec<char>,
   seq: usize,
}
const WORD_COUNT: usize = 10;

pub const MIN_WORD_SIZE: usize = 3;

pub const MAX_WORD_SIZE: usize = 7;

impl LetterGenerator {

   // Generate a sequence of 25 words from the random list. This will be the daily challenge.
   pub fn new(dictionary: Vec<&'static str>, seed_bytes: [u8; 32]) -> LetterGenerator {
      // Initialize the PRNG with the seed value
      let mut rng = rand::rngs::StdRng::from_seed(seed_bytes);

      let mut words = vec![];
      let mut i = 0;
      let mut max_ws = MAX_WORD_SIZE;
      while i < WORD_COUNT {
         let w = dictionary[rng.gen_range(0..dictionary.len())];
         if w.len() < MIN_WORD_SIZE || w.len() > max_ws {
            continue
         }
         i += 1;
         words.push(String::from(w))
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
}

// "RWILEDD" recursion test case.
// "CATDOGANDBUTFOR"
const TEST_LETTERS: &str = "CATDOGANDBUTFOR";

#[derive(Serialize, Deserialize)]
pub struct TestGenerator {
   idx: usize
}

impl TestGenerator {
   pub fn new(words: Vec<&str>, seed_bytes: [u8; 32]) -> TestGenerator {
      TestGenerator{
         idx: 0,
      }
   }
}
impl Generator for TestGenerator {


   fn next_letter(&mut self) -> Option<char>{
      self.idx += 1;
      if self.idx ==TEST_LETTERS.len()+1 {
         return None
      }
      return Some(TEST_LETTERS.chars().nth(self.idx - 1).unwrap())
   }

   fn next_n_letters(&mut self, n: usize) -> Vec<char>{
      let substring = &TEST_LETTERS[self.idx..std::cmp::min(self.idx + n, TEST_LETTERS.len())];
      substring.chars().collect()
   }

   fn num_letters_left(&mut self) -> usize {
      return TEST_LETTERS.len() - self.idx
   }
}

pub trait Generator {
   fn next_letter(&mut self) -> Option<char>;

   fn next_n_letters(&mut self, n: usize) -> Vec<char>;

   fn num_letters_left(&mut self) -> usize;
}

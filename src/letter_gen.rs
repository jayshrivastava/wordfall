use rand::Rng;

#[derive(Copy, Clone)]
pub struct LetterGenerator {}

pub const LETTERS: [char; 26] = [
   'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
   'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'
];

impl LetterGenerator {
   pub fn new() -> LetterGenerator {
      LetterGenerator{
      }
   }
}

impl Generator for LetterGenerator {
   fn next_letter (&mut self) -> char{
      LETTERS[ rand::thread_rng().gen_range(0..=25)]
   }
}


const TEST_LETTERS: &str = "CATDOGANDBUTSOLDCOUNTMATH";
#[derive(Copy, Clone)]
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


   fn next_letter(&mut self) -> char{
      self.idx += 1;
      return TEST_LETTERS.chars().nth(self.idx - 1).unwrap()
   }
}

pub trait Generator {
   fn next_letter(&mut self) -> char;
}

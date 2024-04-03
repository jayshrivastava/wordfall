use rand::Rng;

#[derive(Copy, Clone)]
pub struct LetterGenerator {}

pub const LETTERS: [&str; 26] = [
   "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M",
   "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z"
];

impl LetterGenerator {
   pub fn next_letter(self) -> &'static str{
      LETTERS[ rand::thread_rng().gen_range(0..=25)]
   }
}
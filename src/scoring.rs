use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    static ref SCRABBLE_POINTS: HashMap<char, u32> = {
        let mut map = HashMap::new();
        map.insert('A', 1);
        map.insert('E', 1);
        map.insert('I', 1);
        map.insert('O', 1);
        map.insert('U', 1);
        map.insert('L', 1);
        map.insert('N', 1);
        map.insert('S', 1);
        map.insert('T', 1);
        map.insert('R', 1);
        map.insert('D', 2);
        map.insert('G', 2);
        map.insert('B', 3);
        map.insert('C', 3);
        map.insert('M', 3);
        map.insert('P', 3);
        map.insert('F', 4);
        map.insert('H', 4);
        map.insert('V', 4);
        map.insert('W', 4);
        map.insert('Y', 4);
        map.insert('K', 5);
        map.insert('J', 8);
        map.insert('X', 8);
        map.insert('Q', 10);
        map.insert('Z', 10);
        map
    };
}

pub fn get_score_single(word: &str) -> u32 {
    let mut sc = 0;
    for c in word.chars() {
        sc += SCRABBLE_POINTS.get(&c).unwrap()
    }
    return sc
}
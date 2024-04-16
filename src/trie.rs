use std::ops::{Deref, DerefMut};
use leptos::logging::log;

fn idx(c: char) -> usize {
    return (c as usize) - ('A' as usize)
}
pub struct TrieNode {
    children: Vec<Option<Box<TrieNode>>>,
    word: &'static str,
    terminal: bool,
}

impl TrieNode {
    pub fn new() -> Self {
        let mut t = TrieNode {
            children: Vec::with_capacity(26),
            word: "",
            terminal: false,
        };
        for _ in 0..26 {
            t.children.push(None);
        }
        t
    }

    fn add(&mut self, c: char) {
        self.children[idx(c)] = Some(Box::new(TrieNode::new()))
    }

    fn mark_terminal(&mut self, word: &'static str) {
        self.terminal = true;
        self.word = word
    }

    pub fn terminal(& self) -> bool {
        return self.terminal
    }

    pub fn get_word(& self) -> &'static str {
        return self.word
    }


    pub fn has_next(&self, c: char) -> bool {
        return self.children[idx(c)].is_some()
    }
    pub fn next_mut(&mut self, c: char) -> &mut TrieNode {
        return self.children[idx(c)].as_mut().unwrap().deref_mut()
    }
    pub fn next(&self, c: char) -> &TrieNode {
        return self.children[idx(c)].as_ref().unwrap().deref()
    }
    pub fn add_word(&mut self, word: &'static str) {
        log!("{:?}", word);
        let mut trav = self;
        for c in word.chars() {
            if !trav.has_next(c.clone()) {
                trav.add(c.clone())
            }
            trav = trav.next_mut(c.clone())
        }
        trav.mark_terminal(word);
    }
}


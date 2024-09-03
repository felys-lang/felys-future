use std::collections::HashMap;

use crate::ast::{CacheResult, CacheType};

#[derive(Default)]
pub struct Parser {
    pub stream: Stream,
    pub cache: Cache,
}

impl Parser {
    pub fn expect(&mut self, s: &'static str) -> Option<&'static str> {
        let pos = self.stream.mark();
        for ch in s.chars() {
            let sn = self.stream.next();
            if !matches!(sn, Some(d) if d == ch) {
                self.stream.reset(pos);
                return None;
            }
        }
        Some(s)
    }

    pub fn pla(&mut self, ch: char) -> Option<char> {
        let pos = self.stream.mark();
        let saw = self.stream.next();
        self.stream.reset(pos);
        if Some(ch) == saw {
            saw
        } else {
            None
        }
    }

    pub fn nla(&mut self, ch: char) -> Option<char> {
        let pos = self.stream.mark();
        let saw = self.stream.next();
        self.stream.reset(pos);
        if Some(ch) != saw {
            saw
        } else {
            None
        }
    }
}

#[derive(Default)]
pub struct Stream {
    body: String,
    cursor: usize,
}

impl Stream {
    pub fn mark(&self) -> usize {
        self.cursor
    }

    pub fn reset(&mut self, pos: usize) {
        self.cursor = pos
    }

    pub fn next(&mut self) -> Option<char> {
        let ch = self.body.chars().nth(self.cursor);
        self.cursor += 1;
        ch
    }
}

#[derive(Default)]
pub struct Cache {
    body: HashMap<(usize, CacheType), (usize, CacheResult)>,
    verbose: bool,
    hit: usize,
}

impl Cache {
    pub fn get(&mut self, pos: usize, ct: CacheType) -> Option<(usize, CacheResult)> {
        if let Some(res) = self.body.get(&(pos, ct)) {
            if self.verbose {
                let (end, cr) = res;
                println!("{}\t{}\t{:?} => {:?}", pos, end, ct, cr)
            }
            self.hit += 1;
            Some(res.clone())
        } else {
            None
        }
    }

    pub fn insert(&mut self, pos: usize, ct: CacheType, end: usize, cr: CacheResult) {
        if self.verbose {
            println!("{}\t{}\t{:?} => {:?}", pos, end, ct, cr)
        }
        if self.body.insert((pos, ct), (end, cr)).is_some() {
            panic!("cache conflicted")
        }
    }
}

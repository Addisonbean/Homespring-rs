use std::slice;
use std::str::{self, Split};

pub struct HomespringSplit<'a> {
    split: Split<'a, char>,
    next_str: Option<&'a str>,
}

impl<'a> HomespringSplit<'a> {
    pub fn new(string: &str) -> HomespringSplit {
        HomespringSplit {
            split: string.split(' '),
            next_str: None,
        }
    }
}

impl<'a> Iterator for HomespringSplit<'a> {
    type Item = &'a str;
    // TODO: handle the ` .` escape code
    fn next(&mut self) -> Option<&'a str> {
        let curr = self.next_str.or_else(|| self.split.next());
        let mut curr = match curr {
            Some(s) => s,
            None => return None,
        };
        let start_ptr = curr.as_ptr();
        self.next_str = self.split.next();
        // TODO: handle invalid tokens and crap, like if the prog ends with `.`, or has a toke like
        // `ab.cd`
        while curr.chars().last() == Some('.') || self.next_str.and_then(|s| s.chars().nth(0)) == Some('.') {
            match self.next_str {
                Some(s) => curr = s,
                None => break,
            }
            self.next_str = self.split.next();
        }
        let str_len = curr.as_ptr() as usize - start_ptr as usize + curr.chars().count();
        unsafe {
            let s_slice = slice::from_raw_parts(start_ptr, str_len);
            curr = str::from_utf8(s_slice).unwrap();
        }
        Some(curr)
    }
}

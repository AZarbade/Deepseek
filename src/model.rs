use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub type TermFreq = HashMap<String, usize>;
pub type DocFreq = HashMap<String, usize>;
pub type TermFreqPerDoc = HashMap<PathBuf, (usize, TermFreq)>;

#[derive(Default, Deserialize, Serialize)]
pub struct Model {
    pub tfpd: TermFreqPerDoc,
    pub df: DocFreq,
}

// refer wiki for following calculations reference
// https://en.wikipedia.org/wiki/Tf%E2%80%93idf
pub fn compute_tf(t: &str, n: usize, d: &TermFreq) -> f32 {
    let nume = d.get(t).cloned().unwrap_or(0) as f32;
    let deno = n as f32;
    // WARN: following value is hardcoded for now.
    let const_k: f32 = 0.5; // this constant is to remove bias towards longer documents.
    const_k + (const_k * (nume / deno))
}

pub fn compute_idf(t: &str, n: usize, df: &DocFreq) -> f32 {
    let nume = n as f32;
    let deno = df.get(t).cloned().unwrap_or(1) as f32;
    (nume / deno).log10()
}

pub struct Lexer<'a> {
    content: &'a [char],
}

impl<'a> Lexer<'a> {
    pub fn new(content: &'a [char]) -> Self {
        Self { content }
    }

    fn trim_left(&mut self) {
        while self.content.len() > 0 && self.content[0].is_whitespace() {
            self.content = &self.content[1..];
        }
    }

    fn chop(&mut self, n: usize) -> &'a [char] {
        let token = &self.content[0..n];
        self.content = &self.content[n..];
        token
    }

    fn chop_while<P>(&mut self, mut predicate: P) -> &'a [char]
    where
        P: FnMut(&char) -> bool,
    {
        let mut n = 0;
        while n < self.content.len() && predicate(&self.content[n]) {
            n += 1;
        }
        self.chop(n)
    }

    pub fn next_token(&mut self) -> Option<String> {
        self.trim_left();
        if self.content.len() == 0 {
            return None;
        }

        if self.content[0].is_numeric() {
            return Some(self.chop_while(|x| x.is_numeric()).iter().collect());
        }

        if self.content[0].is_alphabetic() {
            return Some(
                self.chop_while(|x| x.is_alphabetic())
                    .iter()
                    .map(|x| x.to_ascii_uppercase())
                    .collect(),
            );
        }
        return Some(self.chop(1).iter().collect());
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

pub fn search_query<'a>(model: &'a Model, query: &'a [char]) -> Vec<(&'a Path, f32)> {
    let mut result = Vec::<(&Path, f32)>::new();
    for (path, (n, tf_table)) in &model.tfpd {
        let mut rank = 0f32;
        for token in Lexer::new(&query) {
            rank +=
                compute_tf(&token, *n, tf_table) * compute_idf(&token, model.tfpd.len(), &model.df);
        }
        if !rank.is_nan() {
            result.push((path, rank));
        }
    }
    result.sort_by(|(_, a), (_, b)| {
        a.partial_cmp(b)
            .expect(&format!("{a} and {b} are not comparable"))
    });
    result.reverse();
    result
}

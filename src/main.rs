use poppler::PopplerDocument;
use std::collections::HashMap;
use std::fs::{self, File};
use std::process::exit;
use std::{path::Path, path::PathBuf};

#[derive(Debug)]
struct Lexer<'a> {
    content: &'a [char],
}

impl<'a> Lexer<'a> {
    fn new(content: &'a [char]) -> Self {
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

    fn next_token(&mut self) -> Option<&'a [char]> {
        self.trim_left();
        if self.content.len() == 0 {
            return None;
        }

        if self.content[0].is_numeric() {
            return Some(self.chop_while(|x| x.is_numeric()));
        }

        if self.content[0].is_alphabetic() {
            return Some(self.chop_while(|x| x.is_alphabetic()));
        }
        return Some(self.chop(1));
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = &'a [char];
    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

fn _index_document(_content: &str) -> HashMap<String, usize> {
    todo!();
}
fn parse_entire_pdf_file(file_path: &Path) -> Result<String, ()> {
    let pdf = PopplerDocument::new_from_file(&file_path, Some("")).map_err(|err| {
        eprintln!(
            "ERROR: could not read file {file_path}: {err}",
            file_path = file_path.display()
        );
    })?;

    let mut buffer = String::new();
    let n = pdf.get_n_pages();
    for i in 0..n {
        let page = pdf
            .get_page(i)
            .expect("{i} is within the bounds of the range of the page");

        if let Some(content) = page.get_text() {
            buffer.push_str(content);
            buffer.push(' ');
        }
    }

    Ok(buffer)
}

type TermFreq = HashMap<String, usize>;
type TermFreqIndex = HashMap<PathBuf, TermFreq>;

fn main() -> std::io::Result<()> {
    let index_path = "index.json";
    let index_file = File::open(index_path)?;
    println!("Reading {index_path} index file..");
    let tf_index: TermFreqIndex = serde_json::from_reader(index_file)?;
    println!(
        "{index_path} contains {count} files",
        count = tf_index.len()
    );

    Ok(())
}

fn main2() -> std::io::Result<()> {
    let dir_path = "pdf";
    let _top_n = 20;
    let mut tf_index = TermFreqIndex::new();

    for entry in fs::read_dir(&dir_path)? {
        let entry_path = entry?.path();
        let content = match parse_entire_pdf_file(&entry_path) {
            Ok(string) => string.chars().collect::<Vec<_>>(),
            Err(err) => {
                eprintln!("Error parsing PDF file: {err:?}");
                exit(1)
            }
        };
        println!("Indexing {entry_path:?}");

        let mut tf = TermFreq::new();

        for token in Lexer::new(&content) {
            let term = token
                .iter()
                .map(|x| x.to_ascii_uppercase())
                .collect::<String>();

            if let Some(freq) = tf.get_mut(&term) {
                *freq += 1;
            } else {
                tf.insert(term, 1);
            }
        }

        let mut stats = tf.iter().collect::<Vec<_>>();
        stats.sort_by_key(|(_, f)| *f);
        stats.reverse();

        tf_index.insert(entry_path, tf);
    }

    let index_path = "index.json";
    println!("Saving index...");
    let index_file = match File::create(index_path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("ERROR: could not create the {index_path}: {err}");
            exit(1)
        }
    };
    serde_json::to_writer(index_file, &tf_index).unwrap_or_else(|err| {
        eprintln!("ERROR: serde: {err}");
        exit(1)
    });

    Ok(())
}

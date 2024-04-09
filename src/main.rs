use poppler::PopplerDocument;
use std::collections::HashMap;
use std::env;
use std::fs::{self, File};
use std::process::exit;
use std::{path::Path, path::PathBuf};

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

fn parse_entire_pdf_file(file_path: &Path) -> Result<String, ()> {
    let pdf = match PopplerDocument::new_from_file(&file_path, Some("")) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("ERROR: could not read file {file_path:?}: {err}");
            exit(1)
        }
    };

    let mut buffer = String::new();
    for i in 0..pdf.get_n_pages() {
        let page = pdf
            .get_page(i)
            .expect("{i} is within the bounds of the range of the page");

        if let Some(content) = page.get_text() {
            buffer.push_str(content);
            buffer.push(' ');
        }
    }

    Some(buffer)
}

type TermFreq = HashMap<String, usize>;
type TermFreqIndex = HashMap<PathBuf, TermFreq>;

fn check_index(index_path: &str) -> std::io::Result<()> {
    let index_file = match File::open(index_path) {
        Ok(index_file) => index_file,
        Err(err) => {
            eprintln!("ERROR: could not open index file {index_path}: {err}");
            exit(1)
        }
    };
    println!("Reading {index_path} index file...");
    let tf_index: TermFreqIndex = match serde_json::from_reader(index_file) {
        Ok(tf_index) => tf_index,
        Err(err) => {
            eprintln!("ERROR: could not parse index file {index_path}: {err}");
            exit(1)
        }
    };

    println!(
        "{index_path} contains {count} files",
        count = tf_index.len()
    );

    Ok(())
}

fn save_tf_index(tf_index: TermFreqIndex, index_path: &str) -> std::io::Result<(), ()> {
    println!("Saving {index_path}...");

    let index_file = match File::create(index_path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("ERROR: could not create the {index_path}: {err}");
            exit(1)
        }
    };

    match serde_json::to_writer(index_file, &tf_index) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("ERROR: serde error: could not write to {index_path}: {err}");
            exit(1);
        }
    };

    Ok(())
}

fn tf_index_folder(dir_path: &str) -> Result<TermFreqIndex, ()> {
    let dir = match fs::read_dir(dir_path) {
        Ok(dir) => dir,
        Err(err) => {
            eprintln!("ERROR: could not open directory {dir_path}: {err}");
            exit(1);
        }
    };

    let mut tf_index = TermFreqIndex::new();

    'next_file: for file in dir {
        let file_path = match file.path() {
            Ok(file_path) => file_path,
            Err(err) => {
                eprintln!("ERROR: could not read next file in directory {dir_path}: {err}");
                exit(1);
            }
        };

        println!("Indexing {file_path:?}...");

        let content = match parse_entire_pdf_file(&file_path) {
            Ok(content) => content.chars().collect::<Vec<_>>(),
            Err(()) => {
                println!("WARN: could not index {file_path:?}");
                continue 'next_file;
            }
        };

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

        tf_index.insert(file_path, tf);
    }

    Ok(tf_index)
}

fn main() {
    let mut args = env::args();
    let _program = args.next().expect("path to program is provided");
    // TODO: handle error
    let subcommand = args.next().unwrap_or_else(|| {
        println!("ERROR: no subcommand is provided!");
        exit(1);
    });

    match subcommand.as_str() {
        "index" => {
            // TODO: handle error
            let dir_path = args.next().unwrap_or_else(|| {
                println!("ERROR: no directory is provided for {subcommand} subcommand");
                exit(1);
            });

            // TODO: handle error
            let tf_index = tf_index_folder(&dir_path);
            save_tf_index(&tf_index, "index.json")?;
        }
        "search" => {
            // TODO: handle error
            let index_path = args.next().unwrap_or_else(|| {
                println!("ERROR: no path to index is provided for {subcommand} subcommand");
                exit(1);
            });

            // TODO: handle error
            check_index(&index_path).unwrap_or_else(|err| {
                println!("ERROR: could not check index file {index_path}: {err}");
                exit(1);
            });
        }
        _ => {
            println!("ERROR: unknown subcommand {subcommand}");
            return Err(());
        }
    }

    Ok(())
}

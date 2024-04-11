use poppler::PopplerDocument;
use std::collections::HashMap;
use std::env;
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::result::Result;
use std::str;
use tiny_http::{Header, Method, Request, Response, Server, StatusCode};

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
    let pdf = PopplerDocument::new_from_file(&file_path, Some("")).map_err(|err| {
        eprintln!("ERROR: could not read file {file_path:?}: {err}");
    })?;

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

    Ok(buffer)
}

type TermFreq = HashMap<String, usize>;
type TermFreqIndex = HashMap<PathBuf, TermFreq>;

fn check_index(index_path: &str) -> Result<(), ()> {
    println!("Reading {index_path} index file...");

    let index_file = File::open(index_path).map_err(|err| {
        eprintln!("ERROR: could not open index file {index_path}: {err}");
    })?;

    let tf_index: TermFreqIndex = serde_json::from_reader(index_file).map_err(|err| {
        eprintln!("ERROR: could not parse index file {index_path}: {err}");
    })?;

    println!(
        "{index_path} contains {count} files",
        count = tf_index.len()
    );

    Ok(())
}

fn save_tf_index(tf_index: TermFreqIndex, index_path: &str) -> Result<(), ()> {
    println!("Saving {index_path}...");

    let index_file = File::create(index_path).map_err(|err| {
        eprintln!("ERROR: could not create the {index_path}: {err}");
    })?;

    serde_json::to_writer(index_file, &tf_index).map_err(|err| {
        eprintln!("ERROR: serde error: could not write to {index_path}: {err}");
    })?;

    Ok(())
}

fn tf_index_folder(dir_path: &Path, tf_index: &mut TermFreqIndex) -> Result<(), ()> {
    let dir = fs::read_dir(dir_path).map_err(|err| {
        eprintln!(
            "ERROR: could not open directory {dir_path}: {err}",
            dir_path = dir_path.display()
        );
    })?;

    'next_file: for file in dir {
        let file = file.map_err(|err| {
            eprintln!(
                "ERROR: could not read next file in directory {dir_path} during indexing: {err}",
                dir_path = dir_path.display()
            );
        })?;

        let file_path = file.path();

        let file_type = file.file_type().map_err(|err| {
            eprintln!(
                "ERROR: could not determine type of file {file_path}: {err}",
                file_path = file_path.display()
            );
        })?;

        if file_type.is_dir() {
            tf_index_folder(&file_path, tf_index)?;
            continue 'next_file;
        }

        println!("Indexing {:?}...", &file_path);

        let content = match parse_entire_pdf_file(&file_path) {
            Ok(content) => content.chars().collect::<Vec<_>>(),
            Err(()) => continue 'next_file,
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

        tf_index.insert(file_path, tf);
    }

    Ok(())
}

fn usage(program: &str) {
    eprintln!("Usage: {program} [SUBCOMMAND] [OPTIONS]");
    eprintln!("Subcommands:");
    eprintln!("- index <folder>: index the <folder> and save the index to index.json file");
    eprintln!("- search <index-file> [index.json]: check how many documents are indexed in the file (searching is not implemented yet)");
    eprintln!("- serve <port> [9090]: start local http server with web interface");
}

fn serve_static_file(request: Request, file_path: &str, content_type: &str) -> Result<(), ()> {
    let content_type_header = Header::from_bytes("Content-Type", content_type).expect("some some");

    let file = File::open(file_path).map_err(|err| {
        eprintln!("ERROR: could not sever file: {file_path}: {err}");
    })?;

    let rq = Response::from_file(file).with_header(content_type_header);
    request.respond(rq).map_err(|err| {
        eprintln!("ERROR: could not serve static file {file_path}: {err}");
    })
}

fn server_404(request: Request) -> Result<(), ()> {
    request
        .respond(Response::from_string("404").with_status_code(StatusCode(404)))
        .map_err(|err| {
            eprintln!("ERROR: could not serve a request: {err}");
        })
}

fn server_request(mut request: Request) -> Result<(), ()> {
    println!(
        "INFO: received request! method: {:?}, url: {:?}",
        request.method(),
        request.url()
    );

    match (request.method(), request.url()) {
        (Method::Post, "/api/search") => {
            let mut buf = Vec::new();
            let _ = request.as_reader().read_to_end(&mut buf);
            let body = str::from_utf8(&buf)
                .map_err(|err| {
                    eprintln!("ERROR: could not interpret body: {err}");
                })?
                .chars()
                .collect::<Vec<_>>();

            for token in Lexer::new(&body) {
                println!("{token:?}");
            }

            request.respond(Response::from_string("ok")).map_err(|err| {
                eprintln!("ERROR: {err}");
            })
        }

        (Method::Get, "/index.js") => {
            serve_static_file(request, "index.js", "text/javascript; charset=utf-8")
        }

        (Method::Get, "/") | (Method::Get, "/index.html") => {
            serve_static_file(request, "index.html", "text/html; charset=utf-8")
        }

        _ => server_404(request),
    }
}

fn entry() -> Result<(), ()> {
    let mut args = env::args();
    let program = args.next().expect("path to program is provided");

    let subcommand = args.next().ok_or_else(|| {
        usage(&program);
        eprintln!("ERROR: no subcommand is provided");
    })?;

    match subcommand.as_str() {
        "index" => {
            let dir_path = args.next().ok_or_else(|| {
                usage(&program);
                eprintln!("ERROR: no directory is provided for {subcommand} subcommand");
            })?;

            let mut tf_index = TermFreqIndex::new();
            tf_index_folder(Path::new(&dir_path), &mut tf_index)?;
            save_tf_index(tf_index, "index.json")?;
        }
        "search" => {
            let index_path = args.next().ok_or_else(|| {
                usage(&program);
                eprintln!("ERROR: no path to index is provided for {subcommand} subcommand");
            })?;

            check_index(&index_path)?;
        }
        "serve" => {
            let port = args.next().unwrap_or("9090".to_string());
            let server = Server::http(&format!("0.0.0.0:{}", port)).map_err(|err| {
                eprintln!("ERROR: could not start HTTP server on port {port}: {err}");
            })?;

            println!(
                "{}",
                format!("INFO: listining at http://localhost:{}", port)
            );

            for request in server.incoming_requests() {
                let _ = server_request(request);
            }
        }
        _ => {
            usage(&program);
            eprintln!("ERROR: unknown subcommand {subcommand}");
            return Err(());
        }
    }

    Ok(())
}

fn main() -> ExitCode {
    match entry() {
        Ok(()) => ExitCode::SUCCESS,
        Err(()) => ExitCode::FAILURE,
    }
}

use poppler::Document;
use std::env;
use std::fs::{self, File};
use std::io::{BufWriter, Read};
use std::path::Path;
use std::process::ExitCode;
use std::result::Result;
use std::str;

mod model;
use model::*;
mod lexer;
mod server;
pub mod snowball;

fn parse_entire_txt_file(file_path: &Path) -> Result<String, ()> {
    fs::read_to_string(file_path).map_err(|err| {
        eprintln!(
            "ERROR: coult not open file {file_path}: {err}",
            file_path = file_path.display()
        );
    })
}

fn parse_entire_pdf_file(file_path: &Path) -> Result<String, ()> {
    let mut content = Vec::new();
    File::open(file_path)
        .and_then(|mut file| file.read_to_end(&mut content))
        .map_err(|err| {
            eprintln!(
                "ERROR: could not read file {file_path}: {err}",
                file_path = file_path.display()
            );
        })?;

    let pdf = Document::from_data(&content, None).map_err(|err| {
        eprintln!("ERROR: could not read file {file_path:?}: {err}");
    })?;

    let mut buffer = String::new();
    for i in 0..pdf.n_pages() {
        let page = pdf
            .page(i)
            .expect("{i} is within the bounds of the range of the page");

        if let Some(content) = page.text() {
            buffer.push_str(content.as_str());
            buffer.push(' ');
        }
    }

    Ok(buffer)
}

fn parse_entire_file_by_extension(file_path: &Path) -> Result<String, ()> {
    let extension = file_path
        .extension()
        .ok_or_else(|| {
            eprintln!(
                "ERROR: can't detect file type of {file_path} without extension",
                file_path = file_path.display()
            );
        })?
        .to_string_lossy();
    match extension.as_ref() {
        "pdf" => parse_entire_pdf_file(file_path),
        // TODO: specialized parser for markdown files
        "txt" | "md" => parse_entire_txt_file(file_path),
        _ => {
            eprintln!(
                "ERROR: can't detect file type of {file_path}: unsupported extension {extension}",
                file_path = file_path.display(),
                extension = extension
            );
            Err(())
        }
    }
}

fn save_model_as_json(model: &InMemoryModel, index_path: &str) -> Result<(), ()> {
    println!("Saving {index_path}...");

    let index_file = File::create(index_path).map_err(|err| {
        eprintln!("ERROR: could not create {index_path}: {err}");
    })?;

    serde_json::to_writer(BufWriter::new(index_file), &model).map_err(|err| {
        eprintln!("ERROR: serde error: could not write to {index_path}: {err}");
    })?;

    Ok(())
}

fn add_folder_to_model(
    dir_path: &Path,
    model: &mut dyn Model,
    skipped: &mut usize,
) -> Result<(), ()> {
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
        let last_modified = file
            .metadata()
            .map_err(|err| {
                eprintln!("ERROR: could not get metadata of file {file_path:?}: {err}");
            })?
            .modified()
            .map_err(|err| {
                eprintln!("ERROR: could not get last modified data for {file_path:?}: {err}");
            })?;

        let file_type = file.file_type().map_err(|err| {
            eprintln!(
                "ERROR: could not determine type of file {file_path}: {err}",
                file_path = file_path.display()
            );
        })?;

        if file_type.is_dir() {
            add_folder_to_model(&file_path, model, skipped)?;
            continue 'next_file;
        }

        println!("Indexing {:?}...", &file_path);

        let content = match parse_entire_file_by_extension(&file_path) {
            Ok(content) => content.chars().collect::<Vec<_>>(),
            Err(()) => {
                *skipped += 1;
                continue 'next_file;
            }
        };

        model.add_document(file_path, last_modified, &content)?;
    }

    Ok(())
}

fn usage(program: &str) {
    eprintln!("Usage: {program} [SUBCOMMAND] [OPTIONS]");
    eprintln!("Subcommands:");
    eprintln!("- index <folder>: index the <folder> and save the index to index.json file");
    eprintln!("- serve <index-file> [port]: start local http server with web interface");
}

fn entry() -> Result<(), ()> {
    let mut args = env::args();
    let program = args.next().expect("path to program is provided");

    let subcommand = args.next().ok_or_else(|| {
        usage(&program);
        eprintln!("ERROR: no subcommand is provided");
    })?;

    match subcommand.as_str() {
        "reindex" => {
            let dir_path = args.next().ok_or_else(|| {
                usage(&program);
                eprintln!("ERROR: no directory is provided for {subcommand} subcommand");
            })?;
            let index_path = "index.json";
            let index_file = File::open(&index_path).map_err(|err| {
                eprintln!("ERROR: could not open index file {index_path}: {err}");
            })?;
            let mut model: InMemoryModel = serde_json::from_reader(index_file).map_err(|err| {
                eprintln!("ERROR: could not parse index file {index_path}: {err}");
            })?;
            let mut skipped = 0;
            add_folder_to_model(Path::new(&dir_path), &mut model, &mut skipped)?;
            save_model_as_json(&model, index_path)?;
            println!("Skipped {skipped} files.");
            Ok(())
        }
        "index" => {
            let dir_path = args.next().ok_or_else(|| {
                usage(&program);
                eprintln!("ERROR: no directory is provided for {subcommand} subcommand");
            })?;

            let mut skipped = 0;

            let index_path = "index.json";
            let mut model = Default::default();
            add_folder_to_model(Path::new(&dir_path), &mut model, &mut skipped)?;
            save_model_as_json(&model, index_path)?;

            println!("Skipped {skipped} files.");
            Ok(())
        }
        "serve" => {
            let index_path = args.next().ok_or_else(|| {
                usage(&program);
                eprintln!("ERROR: no path to index is provided for {subcommand} subcommand");
            })?;

            let port = args.next().unwrap_or("9090".to_string());
            let address = format!("0.0.0.0:{}", port);

            let index_file = File::open(&index_path).map_err(|err| {
                eprintln!("ERROR: could not open index file {index_path}: {err}");
            })?;

            let model: InMemoryModel = serde_json::from_reader(index_file).map_err(|err| {
                eprintln!("ERROR: could not parse index file {index_path}: {err}");
            })?;
            server::start(&address, &model)
        }
        _ => {
            usage(&program);
            eprintln!("ERROR: unknown subcommand {subcommand}");
            Err(())
        }
    }
}

fn main() -> ExitCode {
    match entry() {
        Ok(()) => ExitCode::SUCCESS,
        Err(()) => ExitCode::FAILURE,
    }
}

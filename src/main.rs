use std::env;
use std::fs::{self, File};
use std::io::BufWriter;
use std::path::Path;
use std::process::ExitCode;
use std::result::Result;
use std::str;

mod model;
use model::*;
mod server;

fn parse_entire_pdf_file(file_path: &Path) -> Result<String, ()> {
    use poppler::Document;
    use std::io::Read;

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

fn add_folder_to_model(dir_path: &Path, model: &mut dyn Model) -> Result<(), ()> {
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
            add_folder_to_model(&file_path, model)?;
            continue 'next_file;
        }

        println!("Indexing {:?}...", &file_path);

        let content = match parse_entire_pdf_file(&file_path) {
            Ok(content) => content.chars().collect::<Vec<_>>(),
            Err(()) => continue 'next_file,
        };

        model.add_document(file_path, &content)?;
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

    let mut subcommand = None;
    let mut use_sqlite_mode = false;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--sqlite" => use_sqlite_mode = true,
            _ => {
                subcommand = Some(arg);
                break;
            }
        }
    }

    let subcommand = subcommand.ok_or_else(|| {
        usage(&program);
        eprintln!("ERROR: no subcommand is provided");
    })?;

    match subcommand.as_str() {
        "index" => {
            let dir_path = args.next().ok_or_else(|| {
                usage(&program);
                eprintln!("ERROR: no directory is provided for {subcommand} subcommand");
            })?;

            if use_sqlite_mode {
                let index_path = "index.db";

                if let Err(err) = fs::remove_file(index_path) {
                    if err.kind() != std::io::ErrorKind::NotFound {
                        eprintln!("ERROR: could not delete file {index_path}: {err}");
                        return Err(());
                    }
                }

                let mut model = SqliteModel::open(Path::new(index_path))?;
                model.begin()?;
                add_folder_to_model(Path::new(&dir_path), &mut model)?;
                model.commit()
            } else {
                let index_path = "index.json";
                let mut model = Default::default();
                add_folder_to_model(Path::new(&dir_path), &mut model)?;
                save_model_as_json(&model, index_path)
            }
        }
        "serve" => {
            let index_path = args.next().ok_or_else(|| {
                usage(&program);
                eprintln!("ERROR: no path to index is provided for {subcommand} subcommand");
            })?;

            let port = args.next().unwrap_or("9090".to_string());
            let address = format!("0.0.0.0:{}", port);

            if use_sqlite_mode {
                let model = SqliteModel::open(Path::new(&index_path))?;
                server::start(&address, &model)
            } else {
                let index_file = File::open(&index_path).map_err(|err| {
                    eprintln!("ERROR: could not open index file {index_path}: {err}");
                })?;

                let model: InMemoryModel = serde_json::from_reader(index_file).map_err(|err| {
                    eprintln!("ERROR: could not parse index file {index_path}: {err}");
                })?;
                server::start(&address, &model)
            }
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

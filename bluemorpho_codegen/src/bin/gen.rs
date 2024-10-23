use std::{fs::File, io::Write, process::exit};

use bluemorpho_codegen::Gen;
use walkdir::WalkDir;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let path = match args.as_slice() {
        [_, path] => path,
        _ => {
            eprintln!("Usage: gen <DIR>");
            exit(1);
        }
    };

    let mut gen = Gen::new();

    for res in WalkDir::new(path) {
        let entry = res.unwrap();

        if !entry.file_type().is_file() {
            continue;
        }

        let Some(file_name) = entry.file_name().to_str() else {
            continue;
        };

        if !file_name.ends_with(".json") {
            continue;
        }

        let f = File::open(entry.path()).expect("couldn't open file");
        let s = std::io::read_to_string(f).expect("couldn't read file");

        std::io::stdout().flush().unwrap();

        gen.add_lexicon(s.as_str());
    }

    gen.generate();
}

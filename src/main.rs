mod args;
mod constants;
mod lexer;
mod reader;
mod token;
use lexer::Lexer;
use reader::TranslationUnit;
use std::{env, process};

fn main() {
    let cli_args = args::Args::new(env::args().collect::<Vec<String>>()).unwrap_or_else(|err| {
        eprintln!("mando error: {}", err);
        process::exit(1);
    });

    if !cli_args.is_valid_file() {
        eprintln!(
            "mando error: {} is not a valid mando file",
            cli_args.get_filename()
        );
        process::exit(1);
    }

    let index = TranslationUnit::new(cli_args.get_filename()).unwrap_or_else(|err| {
        eprintln!("mando error: {}: {}", err, cli_args.get_filename());
        process::exit(1);
    });
    let mut tokenizer = Lexer::new(index.file_content());

    println!("{:?}", tokenizer.get_tokens());
}

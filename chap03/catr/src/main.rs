use clap::{Arg, ArgAction, Command};

// #[derive(Debug, Parser)]
// struct Args {
//     /// Input text
//     #[arg(required(true))]
//     files: Vec<String>,

//     /// Number all output lines
//     #[arg(short('n'))]
//     number_lines: bool,

//     /// Number non-empty output lines
//     #[arg(short('b'))]
//     number_nonblank_lines: bool,
// }

#[derive(Debug)]
struct Args {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

fn get_args() -> Args {
    let matches = Command::new("catr")
        .version("0.1.0")
        .author("Nikolai Schlegel <nikolai.schlegel@gmail.com>")
        .about("Rust version of 'cat'")
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .help("Input files")
                .required(true)
                .num_args(1..),
        )
        .arg(
            Arg::new("number_lines")
                .short('n')
                .action(ArgAction::SetTrue)
                .help("Number all output lines"),
        )
        .arg(
            Arg::new("number_nonblank_lines")
                .short('b')
                .action(ArgAction::SetTrue)
                .help("Number non-empty output lines"),
        )
        .get_matches();

    Args {
        files: matches.get_many("files").unwrap().cloned().collect(),
        number_lines: matches.get_flag("number_lines"),
        number_nonblank_lines: matches.get_flag("number_nonblank_lines"),
    }
}

fn main() {
    let args = get_args();
    println!("{args:#?}")
}

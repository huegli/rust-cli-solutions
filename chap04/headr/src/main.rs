use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of 'head'
struct Args {
    /// Input file(s)
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,

    /// Number of lines
    #[arg(
        value_name = "LINES",
        short('n'),
        long("lines"),
        default_value = "10",
        conflicts_with("bytes")
    )]
    lines: u64,

    /// Number of bytes
    #[arg(value_name = "BYTES", short('c'), long("bytes"))]
    bytes: Option<u64>,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
}

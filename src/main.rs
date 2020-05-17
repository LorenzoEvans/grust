use std::env::args;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}
fn main() {
    let args = Cli::from_args();
    let content = std::fs::read_to_string(&args.path)
        .expect("Could not read file.");
    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("Returned: {}", line);
        }
    }
    println!("It worked!")}
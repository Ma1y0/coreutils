use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(default_value_t = String::new())]
    s: String,

    /// Do not output the trailing newline
    #[arg(short)]
    n: bool,
}

fn main() {
    let args = Args::parse();

    if args.n {
        print!("{}", args.s);
    } else {
        println!("{}", args.s);
    }
}

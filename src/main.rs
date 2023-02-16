use clap::Parser;

#[derive(Parser)]
struct Arguments {
    command: String
}


fn main() {
    let args = Arguments::parse();
    println!("this is the command: {}", args.command);
}

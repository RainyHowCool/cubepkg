use clap::Parser;
use cubepkg::VM;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// binary file
    binary: String,
    /// debug-mode
    #[clap(short, long, default_value = "true")]
    debug: String,
}

fn main() {
    let args = Args::parse();

    let debug: bool = match args.debug.as_str() {
        "true" => true,
        _ => false,
    };

    println!("Binary: {}", &args.binary);
    println!("Debug Mode: {}", args.debug);

    let vm = VM::from_file(1 << 20, debug, args.binary).unwrap();
    vm.run();
}

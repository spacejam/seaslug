use puke::{Args, Interpreter};

fn main() {
    let args = Args::parse();
    Interpreter::run(args);
}

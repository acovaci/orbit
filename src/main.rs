mod cli;
mod python;
mod util;

fn main() {
    util::logger::init();
    cli::main();
}

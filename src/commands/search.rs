use crate::cli::SearchArgs;

pub fn run(args: SearchArgs) {
    println!("Searching {}", args.repo);
}

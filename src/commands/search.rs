use crate::cli::SearchArgs;
use crate::github::release::fetch_releases;

pub fn run(args: SearchArgs) {
    println!("Searching {}", args.repo);
    match fetch_releases(&args.repo) {
        Ok(releases) => {
            println!("Repository: {}\n", args.repo);
            println!("Available releases:");

            for (i, r) in releases.iter().enumerate() {
                if i == 0 {
                    println!("- {} (latest)", r.tag_name);
                } else {
                    println!("- {}", r.tag_name);
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e)
        }
    }
}

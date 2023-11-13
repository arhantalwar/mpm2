use std::{env, process};
use mpm::Config;

#[tokio::main]
async fn main() {

    let args: Vec<String> = env::args().collect();

    if args.contains(&String::from("init")) {
        let _ = Config::init_config(&args).unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {err}");
            process::exit(1);
        });
    }

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let _ = mpm::run(&config).await;

}

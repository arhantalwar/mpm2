use std::{error::Error, fs::{self, OpenOptions}, process, env, io::Write};
use reqwest;

pub struct Config {
    pub repo_name: String,
    pub is_current_dir: bool,
    pub is_private: bool
}

impl Config {

    pub fn build(args: &[String]) -> Result<Config, &'static str> {

        if args.len() < 2 {
            return Err("Not enought arguments passed");
        }

        let repo_name = args[1].clone();
        let is_current_dir = String::from(".");
        let is_private = String::from("-p");

        let config = Config {
            repo_name,
            is_current_dir: args.contains(&is_current_dir),
            is_private: args.contains(&is_private)
        };

        return Ok(config);

    }

    pub fn init_config(args: &[String]) -> Result<(), &'static str> {
        
        let get_user = env::var("USER").unwrap();
        let dir_path = format!("/home/{}/.config/mpm", get_user);

        if let Err(err) = fs::create_dir(&dir_path) {
            //eprint!("Error creating dir: {}", err);
        }

        let file_path = format!("/home/{}/.config/mpm/mpm.conf", get_user);
        let file = fs::File::create(&file_path);

        let mut f = OpenOptions::new()
            .write(true)
            .append(true)
            .open(&file_path)
            .unwrap();

        writeln!(f, "{}", &args[2]).unwrap();
        writeln!(f, "{}", &args[3]).unwrap();
        println!("successfully update username and token");

        Ok(())

    }

}

struct Auth {
    username: String,
    key: String
}

impl Auth {

    fn new() -> Auth {

        let config_file_path = format!("/home/{}/.config/mpm/mpm.conf", env::var("USER").unwrap());

        let content = fs::read_to_string(config_file_path).unwrap_or_else(|err| {
            eprintln!("mpm.conf is not found, please do mpm init <username> <token>");
            process::exit(1);
        });

        let mut lines = content.lines();

        let username = lines.next().unwrap_or_else(|| {
            eprintln!("MPM is not init");
            process::exit(1);
        }).to_string();

        let key = lines.next().unwrap_or_else(|| {
            eprintln!("MPM is not init");
            process::exit(1);
        }).to_string();

        return Auth {
            username,
            key
        };

    }

}

pub async fn run(config: &Config) -> Result<(), Box<dyn Error>> {

    let url = "https://api.github.com/user/repos";

    let auth = Auth::new();

    let json_payload = serde_json::json!({
        "name": &config.repo_name,
        "private": &config.is_private,
    });

    let client = reqwest::Client::new()
        .post(url)
        .basic_auth(&auth.username, Some(&auth.key))
        .header(reqwest::header::USER_AGENT, &config.repo_name)
        .json(&json_payload)
        .send()
        .await?;

    if client.status().is_success() {
        println!("Repo created successfully");
    } else {
        eprintln!("Repo was not created");
        process::exit(1);
    }

    Ok(())

}

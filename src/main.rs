use serde::Deserialize;
use std::{borrow::Borrow, fs};

use clap::Parser;

use std::env;

//TODO: Move thise into seperate types folder
#[derive(Parser, Debug)]
struct CliArguments {
    ticket_number: String,
}

#[derive(Deserialize, Debug)]
struct Ticket {
    key: String,
    fields: Field,
}

#[derive(Deserialize)]
struct Config {
    jira_credentials: JiraCredentials,
}

#[derive(Deserialize)]
struct JiraCredentials {
    username: String,
    token: String,
}

#[derive(Deserialize, Debug)]
struct Field {
    summary: String,
}

#[tokio::main]
async fn main() {
    let args: CliArguments = CliArguments::parse();
    let vars = read_config();
    let client = reqwest::Client::new();
    let strings = format!(
        "{}:{}",
        vars.jira_credentials.username, vars.jira_credentials.token
    );
    let b64 = base64::encode(&strings);
    let ff = format!("Basic {}", b64);
    let url = format!(
        "https://clearscore.atlassian.net/rest/api/latest/issue/{}",
        args.ticket_number
    );

    let response = client
        .get(url)
        .header("Authorization", ff)
        .send()
        .await
        .expect("some")
        .text()
        .await;

    match response {
        Ok(res) => {
            let v = serde_json::from_str::<Ticket>(&res);
            match v {
                Ok(resp) => {
                    let desc: String = resp.fields.summary.to_lowercase().replace(" ", "-");

                    let branch: String = format!("{}-{}", resp.key, desc);
                    println!("{}", branch);
                    if does_git_exist() {
                        println!("{}", branch_git(branch));
                    }
                }
                _ => println!("{}", "No value"),
            }
        }
        Err(_) => println!("{}", "some err"),
    }
}

fn does_git_exist() -> bool {
    return std::process::Command::new("git").output().is_ok();
}

fn branch_git(branch_name: String) -> bool {
    return std::process::Command::new("git")
        .args([
            "checkout",
            "-b",
            branch_name
                .replace(".", "") //TODO: get the replace regex for this [!"#$%&'()*+,.\/:;<=>?@[\]^_`{|}~]
                .replace(",", "")
                .replace(" ", "")
                .borrow(),
        ])
        .spawn()
        .is_ok();
}

fn read_config() -> Config {
    let contents =
        fs::read_to_string("./config.toml").expect("Should have been able to read the file");

    let config: Config = toml::from_str(contents.borrow()).unwrap();
    return config;
}

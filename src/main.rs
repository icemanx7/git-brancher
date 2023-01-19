use serde::Deserialize;
use std::borrow::Borrow;

use clap::Parser;

use std::env;

#[derive(Parser, Debug)]
struct CliArguments {
    ticket_link: String,
}

#[derive(Deserialize, Debug)]
struct Ticket {
    key: String,
    fields: Field,
}

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
    let vars = read_env_var();
    let client = reqwest::Client::new();

    let response = client
        .get(args.ticket_link)
        .basic_auth(vars.username, Some(vars.token))
        .send()
        .await
        .expect("some")
        .text()
        .await;

    match response {
        Ok(res) => {
            let v = serde_json::from_str::<Ticket>(&res);
            println!("{:?}", v);
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
        .args(["checkout", "-b", branch_name.borrow()])
        .spawn()
        .is_ok();
}

fn read_env_var() -> JiraCredentials {
    let jira_token = env::var("JIRA_TOKEN");
    let jira_username = env::var("JIRA_USERNAME");
    return match (jira_token, jira_username) {
        (Ok(token), Ok(username)) => JiraCredentials { username, token },
        _ => JiraCredentials {
            username: "".to_owned(),
            token: "".to_owned(),
        },
    };
}

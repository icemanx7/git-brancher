use std::borrow::Borrow;

use clap::Parser;

#[derive(Parser, Debug)]
struct CliArguments {
    ticket_number: String,
    desciption: String,
}

fn main() {
    let args: CliArguments = CliArguments::parse();
    println!("{:?}", args);

    let desc: String = args.desciption.to_lowercase().replace(" ", "-");

    let branch: String = format!("{}-{}", args.ticket_number, desc);

    println!("{}", branch);

    if does_git_exist() {
        println!("{}", branch_git(branch));
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

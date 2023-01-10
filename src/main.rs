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

    println!("{}", branch)
}

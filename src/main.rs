use clap::{Arg, Command};

fn cli() -> Command<'static> {
    Command::new("strace-log-utils")
        .about("strace(1) log utilities")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("parse")
                .about("Parse strace logs")
                .arg(Arg::new("input").help("Path to logs"))
                .arg_required_else_help(true),
        )
}

fn do_parse(input: &String) {
    println!("{input}")
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("parse", sub_matches)) => {
            do_parse(sub_matches.get_one::<String>("input").expect("required"))
        }
        _ => unreachable!(),
    }
}

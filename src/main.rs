use anyhow::Result;
use std::fs;
use strace_log_utils::parser::parse_log_line;

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

fn do_parse(input: &String) -> Result<()> {
    let contents = fs::read_to_string(input)?;
    let (logs, errors): (Vec<_>, Vec<_>) = contents
        .lines()
        .map(parse_log_line)
        .partition(Result::is_ok);
    dbg!(logs);
    dbg!(errors);
    Ok(())
}

fn main() -> Result<()> {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("parse", sub_matches)) => {
            do_parse(sub_matches.get_one::<String>("input").expect("required"))
        }
        _ => unreachable!(),
    }
}

use clap::{arg, command, value_parser, Arg, ArgAction, Command, ArgMatches, Args, Subcommand};

mod cli;

fn main() {
	let matches = cli::build_cli().get_matches();

	match matches.subcommand() {
		Some(("projects", subcommand)) => {
			let matches = subcommand.get_matches();
		}
		_ => {
			println!("INVALID");
		}
	}
}

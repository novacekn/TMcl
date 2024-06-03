use clap::{arg, command, value_parser, Arg, ArgAction, Command, ArgMatches, Args, Subcommand};

mod cli;

fn main() {
	let matches = cli::build_cli().get_matches();

	match matches.subcommand() {
		Some(("projects", sub_matches)) => {
			let project_cmd = sub_matches.subcommand().unwrap();
			match project_cmd {
				("list", sub_matches) => {
					println!("LISTING PROJECTS");
				}
				("add", sub_matches) => {
					let name = sub_matches.get_one::<String>("name").unwrap();
					let description = sub_matches.get_one::<String>("description").unwrap();
					println!("Name: {}, Description: {}", name, description);
				}
				("edit", sub_matches) => {
					let project_id = sub_matches.get_one::<String>("id").unwrap();
					let name = sub_matches.get_one::<String>("name").unwrap();
					let description = sub_matches.get_one::<String>("description").unwrap();
					println!("ID: {}, Name: {}, Description: {}", project_id, name, description);
				}
				("delete", sub_matches) => {
					let project_id = sub_matches.get_one::<String>("id").unwrap();
					println!("ID: {}", project_id);
				}
				_ => {
					println!("INVALID");
				}
			}
		}
		Some(("categories", sub_matches)) => {
			let category_cmd = sub_matches.subcommand().unwrap();
			match category_cmd {
				("list", sub_matches) => {
					println!("LISTING CATEGORIES");
				}
				("add", sub_matches) => {
					let name = sub_matches.get_one::<String>("name").unwrap();
					println!("Name: {}", name);
				}
				("edit", sub_matches) => {
					let category_id = sub_matches.get_one::<String>("id").unwrap();
					let name = sub_matches.get_one::<String>("name").unwrap();
					println!("ID: {}, Name: {}", category_id, name);
				}
				("delete", sub_matches) => {
					let category_id = sub_matches.get_one::<String>("id").unwrap();
					println!("ID: {}", category_id);
				}
				_ => {
					println!("INVALID");
				}
			}
		}
		Some(("tasks", sub_matches)) => {
			let tasks_cmd = sub_matches.subcommand().unwrap();
			match tasks_cmd {
				("list", sub_matches) => {
					println!("LIST TASKS");
				}
				("add", sub_matches) => {
					let title = sub_matches.get_one::<String>("title").unwrap();
					let description = sub_matches.get_one::<String>("description").unwrap();
					let priority = sub_matches.get_one::<String>("priority").unwrap();
					let due_date = sub_matches.get_one::<String>("due_date").unwrap();
					let project_id = sub_matches.get_one::<String>("project_id").unwrap();
					let category_id = sub_matches.get_one::<String>("category_id").unwrap();

					println!(
						"Title: {}, Description: {}, Priority: {}, Due Date: {}, Project ID: {}, Category ID: {}",
						title,
						description,
						priority,
						due_date,
						project_id,
						category_id
					);
				}
				("edit", sub_matches) => {
					let task_id = sub_matches.get_one::<String>("id").unwrap();
					let title = sub_matches.get_one::<String>("title").unwrap();
					let description = sub_matches.get_one::<String>("description").unwrap();
					let priority = sub_matches.get_one::<String>("priority").unwrap();
					let due_date = sub_matches.get_one::<String>("due_date").unwrap();
					let project_id = sub_matches.get_one::<String>("project_id").unwrap();
					let category_id = sub_matches.get_one::<String>("category_id").unwrap();

					println!(
						"ID: {}, Title: {}, Description: {}, Priority: {}, Due Date: {}, Project ID: {}, Category ID:
						{}",
						task_id,
						title,
						description,
						priority,
						due_date,
						project_id,
						category_id
					);
				}
				("delete", sub_matches) => {
					let task_id = sub_matches.get_one::<String>("id").unwrap();
					println!("ID: {}", task_id);
				}
				_ => {
					println!("INVALID");
				}
			}
		}
		_ => {
			println!("INVALID");
		}
	}
}

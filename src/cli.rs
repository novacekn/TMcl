use clap::{arg, command, value_parser, Arg, ArgAction, Command};

pub fn build_cli() -> Command {
	Command::new("tm")
		.about("A task manager for the command-line")
		.version("1.0")
		.author("Nathan Novacek")
		.subcommand(
			Command::new("projects")
				.about("Manage projects")
				.subcommand(
					Command::new("list")
					.about("List projects")
					.arg(
						Arg::new("all")
							.short('a')
							.long("all")
							.help("Lists all projects (even completed)")
					)
				)
				.subcommand(
					Command::new("add")
					.about("Add a new project")
					.arg(
						Arg::new("name")
						.short('n')
						.long("name")
						.value_name("name")
						.help("Add a name to the project")
					)
					.arg(
						Arg::new("description")
							.short('d')
							.long("description")
							.value_name("description")
							.help("Add a description to the project")
					)
				)
				.subcommand(
					Command::new("edit")
						.about("Edit a project")
						.arg(
							Arg::new("id")
								.short('i')
								.long("id")
								.value_name("project_id")
								.required(true)
								.help("The project's ID")
						)
						.arg(
							Arg::new("name")
								.short('n')
								.long("name")
								.value_name("name")
								.required(false)
								.help("The new name for the project")
						)
						.arg(
							Arg::new("description")
								.short('d')
								.long("description")
								.value_name("description")
								.required(false)
								.help("The new description for the project")
						)
				)
				.subcommand(
					Command::new("delete")
						.about("Delete a project")
						.arg(
							Arg::new("id")
								.short('i')
								.long("id")
								.value_name("project_id")
								.required(true)
								.help("A project ID for deletion")
						)
				)
		)
		.subcommand(
			Command::new("categories")
				.about("Manage categories")
				.subcommand(
					Command::new("list")
						.about("List all categories")
				)
				.subcommand(
					Command::new("add")
						.about("Add a new category")
						.arg(
							Arg::new("name")
								.short('n')
								.long("name")
								.value_name("name")
								.required(true)
								.help("The name of the category")
						)
				)
				.subcommand(
					Command::new("edit")
						.about("Edit a category")
						.arg(
							Arg::new("id")
								.short('i')
								.long("id")
								.value_name("id")
								.required(true)
								.help("The ID of a category")
						)
						.arg(
							Arg::new("name")
								.short('n')
								.long("name")
								.value_name("name")
								.required(true)
								.help("The new name for the category")
						)
				)
				.subcommand(
					Command::new("delete")
						.about("Delete a category")
						.arg(
							Arg::new("id")
								.short('i')
								.long("id")
								.value_name("id")
								.required(true)
								.help("The ID of the category for deletion")
						)
				)
		)
		.subcommand(
			Command::new("tasks")
				.about("Manage tasks")
				.subcommand(
					Command::new("list")
						.about("List tasks")
						.arg(
							Arg::new("all")
								.short('a')
								.long("all")
								.help("List all tasks")
						)
						.arg(
							Arg::new("project-name")
								.long("project-name")
								.help("Search for and list by project name")
						)
						.arg(
							Arg::new("project-id")
								.long("project-id")
								.short('p')
								.help("List tasks by project ID")
						)
						.arg(
							Arg::new("category-name")
								.long("category-name")
								.help("Search for and list by category name")
						)
						.arg(
							Arg::new("category-id")
								.long("category-id")
								.short('c')
								.help("List tasks by category")
						)
				)
				.subcommand(
					Command::new("add")
						.about("Add a task")
						.arg(
							Arg::new("title")
								.short('t')
								.long("title")
								.value_name("title")
								.required(true)
								.help("Add a title to the task")
						)
						.arg(
							Arg::new("description")
								.short('d')
								.long("description")
								.value_name("description")
								.required(true)
								.help("Add a description to the task")
						)
						.arg(
							Arg::new("due")
								.long("due")
								.value_name("due_date")
								.required(false)
								.help("Add a due date to the task")
						)
						.arg(
							Arg::new("priority")
								.short('p')
								.long("priority")
								.value_name("priority")
								.help("Add a priority to the task")
						)
						.arg(
							Arg::new("project-id")
								.long("project-id")
								.value_name("project_id")
								.help("Add a project ID to the task")
						)
						.arg(
							Arg::new("category-id")
								.long("category-id")
								.value_name("category_id")
								.help("Add a category ID to the task")
						)
				)
				.subcommand(
					Command::new("edit")
						.about("Edit a task")
						.arg(
							Arg::new("id")
								.short('i')
								.long("id")
								.value_name("id")
								.required(true)
								.help("The tasks ID")
						)
						.arg(
							Arg::new("title")
								.short('t')
								.long("title")
								.value_name("title")
								.required(false)
								.help("The new title for the task")
						)
						.arg(
							Arg::new("description")
								.short('d')
								.long("description")
								.value_name("description")
								.required(false)
								.help("The new description for the task")
						)
						.arg(
							Arg::new("due")
								.long("due")
								.value_name("due")
								.required(false)
								.help("Change the due date for the task")
						)
						.arg(
							Arg::new("priority")
								.short('p')
								.long("priority")
								.required(false)
								.help("Change the priority for the task")
						)
						.arg(
							Arg::new("project-id")
								.long("project-id")
								.required(false)
								.help("Change the project associated with this task")
						)
						.arg(
							Arg::new("category-id")
								.long("category-id")
								.required(false)
								.help("Change the category associated with this task")
						)
				)
				.subcommand(
					Command::new("delete")
						.about("Delete a task")
						.arg(
							Arg::new("id")
								.short('i')
								.long("id")
								.value_name("id")
								.required(true)
								.help("The tasks ID")
						)
				)
		)
}

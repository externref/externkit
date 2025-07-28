use clap::{value_parser, Arg, ArgMatches, Command};

mod backend;

fn main() {
    let matches = clap::Command::new("externkit")
        .version("0.1.0")
        .author("externref")
        .about("General project management tool by externref.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .color(clap::ColorChoice::Auto)
        .styles(
            clap::builder::Styles::styled()
                .header(clap::builder::styling::AnsiColor::Green.on_default().bold())
                .usage(
                    clap::builder::styling::AnsiColor::Yellow
                        .on_default()
                        .bold(),
                )
                .literal(clap::builder::styling::AnsiColor::Blue.on_default().bold())
                .placeholder(clap::builder::styling::AnsiColor::Cyan.on_default())
                .error(clap::builder::styling::AnsiColor::Red.on_default().bold())
                .valid(clap::builder::styling::AnsiColor::Green.on_default().bold())
                .invalid(clap::builder::styling::AnsiColor::Red.on_default().bold()),
        )
        .subcommand(
            Command::new("env")
                .about("Environment variables management commands")
                .subcommand_required(true)
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("add")
                        .about("Add or set an environment variable")
                        .arg(
                            Arg::new("key")
                                .help("Environment variable name")
                                .required(true)
                                .value_parser(value_parser!(String)),
                        )
                        .arg(
                            Arg::new("value")
                                .help("Environment variable value")
                                .required(true)
                                .value_parser(value_parser!(String)),
                        ),
                )
                .subcommand(
                    Command::new("delete")
                        .about("Delete an environment variable")
                        .arg(
                            Arg::new("key")
                                .help("Environment variable name to delete")
                                .required(true)
                                .value_parser(value_parser!(String)),
                        ),
                )
                .subcommand(
                    Command::new("update")
                        .about("Update an existing environment variable")
                        .arg(
                            Arg::new("key")
                                .help("Environment variable name")
                                .required(true)
                                .value_parser(value_parser!(String)),
                        )
                        .arg(
                            Arg::new("value")
                                .help("New environment variable value")
                                .required(true)
                                .value_parser(value_parser!(String)),
                        ),
                ),
        )
        .subcommand(Command::new("init").about("Initialize the externkit project"));

    let matches = matches.get_matches();
    let project_path = std::path::PathBuf::from("./.externkit");
    match matches.subcommand() {
        Some(("init", _)) => {
            backend::utils::init_project();
        }
        Some((cmd, _)) if cmd == "help" || cmd == "version" => {}
        _ => {
            if !project_path.exists() {
                println!(
                    "{}",
                    colored::Colorize::red(
                        "Project not initialized. Use the `externkit init` command to set up the project."
                    )
                );
                return;
            }

            match matches.subcommand() {
                Some(("env", env_matches)) => {
                    handle_env_var_command(env_matches);
                }
                _ => unreachable!(
                    "Exhausted list of subcommands and subcommand_required prevents `None`"
                ),
            }
        }
    }
}

fn handle_env_var_command(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            let key = sub_matches.get_one::<String>("key").expect("required");
            let value = sub_matches.get_one::<String>("value").expect("required");

            backend::env::add_env_var(key, value);
        }
        Some(("delete", sub_matches)) => {
            let key = sub_matches.get_one::<String>("key").expect("required");

            backend::env::delete_env_var(key);
        }
        Some(("update", sub_matches)) => {
            let key = sub_matches.get_one::<String>("key").expect("required");
            let value = sub_matches.get_one::<String>("value").expect("required");

            backend::env::update_env_var(key, value);
        }
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}

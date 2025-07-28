use colored::Colorize;

fn load_env_vars() -> std::collections::HashMap<String, String> {
    let content =
        std::fs::read_to_string(".externkit/environment_variables.json").unwrap_or_default();
    if content.is_empty() {
        std::collections::HashMap::new()
    } else {
        serde_json::from_str(&content).unwrap_or_default()
    }
}

fn save_env_vars(env_vars: &std::collections::HashMap<String, String>) {
    let content = serde_json::to_string_pretty(env_vars).unwrap();
    std::fs::write(".externkit/environment_variables.json", content).unwrap();
}

pub fn add_env_var(name: &str, value: &str) {
    let mut env_vars = load_env_vars();
    if env_vars.contains_key(name) {
        println!(
            "{}",
            format!(
                "Environment variable '{}' already exists. Use 'update' to change its value.",
                name
            )
            .yellow()
        );
        return;
    }
    if name.is_empty() || value.is_empty() {
        println!(
            "{}",
            "Environment variable name and value cannot be empty.".red()
        );
        return;
    }
    env_vars.insert(name.to_string(), value.to_string());
    save_env_vars(&env_vars);
    println!(
        "{}",
        format!("Added environment variable: {}={}", name, value).green()
    );
}

pub fn delete_env_var(name: &str) {
    let mut env_vars = load_env_vars();
    if !env_vars.contains_key(name) {
        println!(
            "{}",
            format!("Environment variable '{}' does not exist.", name).yellow()
        );
        return;
    }
    if name.is_empty() {
        println!("{}", "Environment variable name cannot be empty.".red());
        return;
    }
    env_vars.remove(name);
    save_env_vars(&env_vars);
    println!(
        "{}",
        format!("Deleted environment variable: {}", name).green()
    );
}
pub fn update_env_var(name: &str, value: &str) {
    let mut env_vars = load_env_vars();
    if !env_vars.contains_key(name) {
        println!(
            "{}",
            format!(
                "Environment variable '{}' does not exist. Use 'add' to create it.",
                name
            )
            .yellow()
        );
        return;
    }
    if name.is_empty() || value.is_empty() {
        println!(
            "{}",
            "Environment variable name and value cannot be empty.".red()
        );
        return;
    }
    env_vars.insert(name.to_string(), value.to_string());
    save_env_vars(&env_vars);
    println!(
        "{}",
        format!("Updated environment variable: {}={}", name, value).green()
    );
}

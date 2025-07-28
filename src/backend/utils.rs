pub fn init_project() {
    let project_path = std::path::PathBuf::from("./.externkit");
    if !project_path.exists() {
        std::fs::create_dir_all(&project_path).expect("Failed to create project directory");
        std::fs::write(project_path.join(".gitignore"), "*")
            .expect("Failed to create gitignore file");

        std::fs::write(project_path.join("environment_variables.json"), "{\n}")
            .expect("Failed to create environment_variables.json file");
    }
}

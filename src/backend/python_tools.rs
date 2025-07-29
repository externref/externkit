pub fn get_pip(python_path: &String) {
    use colored::Colorize;

    println!("{}", "Downloading get-pip.py...".cyan());
    let url = "https://bootstrap.pypa.io/get-pip.py";
    let response = reqwest::blocking::get(url)
        .expect("Failed to fetch get-pip.py")
        .text()
        .expect("Failed to read response text");

    println!("{}", "Writing temporary file...".cyan());
    std::fs::write("get_pip_temp.py", response).unwrap();

    println!(
        "{} {}",
        "Installing pip using:".cyan(),
        python_path.yellow()
    );
    let status = std::process::Command::new(python_path)
        .arg("get_pip_temp.py")
        .status()
        .expect("Failed to run get-pip.py");

    std::fs::remove_file("get_pip_temp.py").expect("Failed to remove temporary file");

    if status.success() {
        println!("{}", "✓ pip installed successfully!".green().bold());
    } else {
        println!("{}", "✗ Failed to install pip".red().bold());
    }
}

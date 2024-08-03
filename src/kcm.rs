use inquire::{InquireError, Select};
use std::process::Command;
use std::str;

pub fn extract_context_names(output: &str) -> Vec<String> {
    let mut context_names = Vec::new();
    let lines: Vec<&str> = output.lines().collect();

    for line in lines.iter().skip(1) {
        // Skip the header line
        let columns: Vec<&str> = line.split_whitespace().collect();
        if let Some(name) = columns.get(1) {
            // Get the second column
            context_names.push(name.to_string());
        }
    }

    context_names
}

pub fn choose_context(options: Vec<String>) {
    let ans: Result<String, InquireError> =
        Select::new("Select the context you want to use", options).prompt();

    match ans {
        Ok(choice) => change_context(choice),
        Err(_) => println!("There was an error, please try again"),
    }
}

fn change_context(option: String) {
    Command::new("kubectl")
        .args(["config", "use-context", &option])
        .output()
        .expect(&format!("Couldn't change context to {}", option));

    let output = Command::new("kubectl")
        .args(["config", "current-context"])
        .output()
        .expect("Failed to get current context");

    if output.status.success() {
        let stdout = str::from_utf8(&output.stdout).expect("Invalid UTF-8 in output");
        println!("Current context is: {}", stdout.trim());
    } else {
        let stderr = str::from_utf8(&output.stderr).expect("Invalid UTF-8 in error output");
        eprintln!("Failed to get current context: {}", stderr);
    }
}

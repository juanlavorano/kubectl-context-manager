mod kcm;

use std::process::Command;
use std::str;

fn main() {
    let output = Command::new("kubectl")
        .args(["config", "get-contexts"])
        .output()
        .expect("Failed to execute kubectl command");

    if output.status.success() {
        let stdout = str::from_utf8(&output.stdout).expect("Invalid UTF-8 in output");
        let context_names = kcm::extract_context_names(stdout);

        kcm::choose_context(context_names);
    } else {
        let stderr = str::from_utf8(&output.stderr).expect("Invalid UTF-8 in error output");
        eprintln!("{}", stderr);
    }
}

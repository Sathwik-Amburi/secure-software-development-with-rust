use std::process::Command;

pub fn make_script_executable(script_path: &str) {
    let output = Command::new("chmod")
        .arg("+x")
        .arg(script_path)
        .output()
        .expect("Failed to set script as executable");

    if !output.status.success() {
        panic!("Failed to set script as executable");
    }
}

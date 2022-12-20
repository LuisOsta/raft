use std::process::{Command, Output};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=data_api/v1/data.proto");
    let Output { status, stderr, .. } = Command::new("buf")
        .arg("generate")
        .arg("--path")
        .arg("data_api")
        .output()?;

    if !status.success() {
        return Err(String::from_utf8_lossy(&stderr).into());
    }

    Ok(())
}

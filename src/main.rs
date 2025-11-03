mod error;

use hyprland::shared::HyprData;
use hyprland::data::{
    Client, Clients, Workspace, Workspaces
};
use std::io::Write;
use std::process::{Command, Stdio};
use std::ffi::CStr;
use std::num::ParseIntError;
use error::AppError;

fn main() -> Result<(), AppError> {
    let clients = Clients::get()?;
    // println!("{:#?}", Clients::get()?);
    // Collect clients, split with new line
    let titles = clients
        .iter()
        .map(|f| f.title.as_str())
        .collect::<Vec<_>>()
        .join("\n");

    let mut child = Command::new("rofi")
        .arg("-dmenu")
        .arg("-format")
        .arg("i")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    {
        let stdin = child.stdin.as_mut().unwrap();
        stdin.write_all(titles.as_bytes())?;
    }

    let output = child.wait_with_output()?;
    let cstr = CStr::from_bytes_until_nul(&output.stdout)?;
    println!("{:?}", String::from_utf8_lossy(&output.stdout));

    Ok(())
}

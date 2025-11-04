mod error;

use hyprland::shared::{HyprData, HyprDataActive};
use hyprland::data::{Clients, Workspace};
use hyprland::dispatch::{Dispatch, DispatchType, WindowIdentifier, WorkspaceIdentifierWithSpecial};
use std::io::Write;
use std::process::{Command, Stdio};
use error::AppError;

fn main() -> Result<(), AppError> {
    let clients = Clients::get()?;
    let active_workspace = Workspace::get_active()?.id;

    // Collect clients into a vector
    let clients_vec: Vec<_> = clients.iter().collect();

    // Find max widths for alignment
    let max_class_width = clients_vec
        .iter()
        .map(|c| c.class.len())
        .max()
        .unwrap_or(0);

    let max_workspace_width = clients_vec
        .iter()
        .map(|c| c.workspace.name.len())
        .max()
        .unwrap_or(0);

    // Collect titles with icons, using format: class  workspace  title\0icon\x1ficon-name
    let titles = clients_vec
        .iter()
        .map(|c| {
            // Use class name as icon (convert to lowercase for standard icon names)
            let icon = c.class.to_lowercase();
            let workspace = &c.workspace.name;

            // Assign color based on workspace ID (Tokyo Night theme from gtk-status-bar)
            let color = match c.workspace.id {
                1 => "#7AA2F7",  // Blue
                2 => "#7DCFFF",  // Light Blue
                3 => "#9ECE6A",  // Green
                4 => "#BB9AF7",  // Purple
                5 => "#F7768E",  // Pink/Red
                6 => "#FF9E66",  // Orange
                7 => "#9D7CD8",  // Purple/Lavender
                8 => "#E0AF68",  // Gold/Yellow
                9 => "#2AC3DE",  // Cyan
                10 => "#0DB9D7", // Teal
                _ => "#43E97B",  // Bright Green (default)
            };

            // Escape markup characters in text
            let class_escaped = c.class.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;");
            let title_escaped = c.title.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;");

            let display = format!(
                "<span color='{}'>  {:<class_w$}  {:<ws_w$}  {}</span>",
                color,
                class_escaped,
                workspace,
                title_escaped,
                class_w = max_class_width,
                ws_w = max_workspace_width
            );
            format!("{}\0icon\x1f{}", display, icon)
        })
        .collect::<Vec<_>>()
        .join("\n");

    let mut child = Command::new("rofi")
        .arg("-dmenu")
        .arg("-show-icons")
        .arg("-markup-rows")
        .arg("-format")
        .arg("i")
        .arg("-p")
        .arg("window")
        .arg("-kb-accept-entry")
        .arg("Return")
        .arg("-kb-accept-alt")
        .arg("")
        .arg("-kb-custom-1")
        .arg("Shift+Return")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    {
        let stdin = child.stdin.as_mut().unwrap();
        stdin.write_all(titles.as_bytes())?;
    }

    let output = child.wait_with_output()?;
    let exit_code = output.status.code().unwrap_or(1);

    // Exit code 1 means cancelled (ESC)
    if exit_code == 1 {
        return Ok(());
    }

    // Parse rofi output as selected index
    let output_str = String::from_utf8(output.stdout)?;
    let selected_index: usize = output_str.trim().parse()?;

    // Get the selected client
    let selected_client = clients_vec[selected_index];
    let window_id = WindowIdentifier::Address(selected_client.address.clone());

    match exit_code {
        0 => {
            // Enter: Focus window
            println!("Focusing window: {}", selected_client.title);
            Dispatch::call(DispatchType::FocusWindow(window_id))?;
        }
        10 => {
            // Shift+Enter: Move window to current workspace
            println!("Moving window to workspace {}: {}", active_workspace, selected_client.title);
            Dispatch::call(DispatchType::MoveToWorkspace(
                WorkspaceIdentifierWithSpecial::Id(active_workspace),
                Some(window_id),
            ))?;
        }
        _ => {
            println!("Unknown exit code: {}", exit_code);
        }
    }

    Ok(())
}

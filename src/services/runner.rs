use std::process::{Command, Stdio};

use crate::data::settings::{RunMode, SettingsInfo, ShellKind, TerminalKind};

/// Starts a saved item in a new terminal. Successful spawning does not mean
/// the command itself succeeded; its output and exit status stay in the terminal.
pub fn run(content: &str, settings: &SettingsInfo) -> Result<(), String> {
    if settings.run_mode != RunMode::NewWindow {
        return Err("Reuse session is not available yet; select New window in Settings".into());
    }

    let shell = match settings.shell {
        ShellKind::Bash => "bash",
    };

    let (terminal, terminal_args): (&str, &[&str]) = match settings.terminal {
        TerminalKind::Gnome => ("gnome-terminal", &["--"]),
        TerminalKind::Konsole => ("konsole", &["-e"]),
        TerminalKind::Kitty => ("kitty", &[]),
        TerminalKind::Xterm => ("xterm", &["-e"]),
    };

    let mut command = Command::new(terminal);
    command.args(terminal_args).arg(shell).arg("-c");

    // Bash does not display PS1 when running `-c`. Print a prompt and the
    // saved text first, then execute the text from a positional argument.
    // This keeps quotes and multiline scripts out of the wrapper source.
    const WRAPPER: &str = r#"
        printf '%s@%s:%s$ %s\n' "$(id -un)" "${HOSTNAME%%.*}" "$PWD" "$1"
        bash -c "$1"
        status=$?
        if [ "$2" = "1" ]; then
            printf '\nExit status: %s\nPress Enter to close...' "$status"
            read -r _
        fi
        exit "$status"
    "#;
    command.arg(WRAPPER)
        .arg("shelf")
        .arg(content)
        .arg(if settings.keep_open { "1" } else { "0" });

    command.stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null());
    command.spawn().map_err(|e| format!("Could not launch {terminal}: {e}"))?;
    Ok(())
}

use anyhow::{Context, Result};
use crossterm::{
    cursor::SetCursorStyle,
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
};
use std::{env, io::stdout};

pub mod ai;
pub mod core;
pub mod i18n;
pub mod process;
pub mod text;
pub mod tui;
pub mod update;

pub fn cli_help_text() -> &'static str {
    "practicode - local-first coding-test practice in your terminal

Usage:
  practicode            Open the TUI
  practicode --smoke    Print the current problem title and exit
  practicode --version  Print the installed version
  practicode --help     Show this help

Inside the TUI:
  Esc then /            Open the command palette
  ?                     Open help
  Ctrl+C                Quit"
}

pub fn run_cli() -> Result<()> {
    let root = env::current_dir().context("read current directory")?;
    let args = env::args().skip(1).collect::<Vec<_>>();
    if args
        .iter()
        .any(|arg| matches!(arg.as_str(), "-h" | "--help"))
    {
        println!("{}", cli_help_text());
        return Ok(());
    }
    if args
        .iter()
        .any(|arg| matches!(arg.as_str(), "-V" | "--version"))
    {
        println!("practicode {}", update::CURRENT_VERSION);
        return Ok(());
    }
    if args.iter().any(|arg| arg == "--smoke") {
        let bank = core::load_bank(&root)?;
        let state = core::load_state(&root, &bank)?;
        let problem = core::problem_by_id(&bank, &state.current_problem).unwrap_or(&bank[0]);
        println!(
            "{}",
            core::localized(&problem.title, &state.settings.ui_language)
        );
        return Ok(());
    }

    let mut app = tui::PracticodeApp::new(root)?;
    let mut terminal = ratatui::init();
    let _ = execute!(stdout(), SetCursorStyle::SteadyBar, EnableMouseCapture);
    let result = app.run(&mut terminal);
    ratatui::restore();
    let _ = execute!(
        stdout(),
        SetCursorStyle::DefaultUserShape,
        DisableMouseCapture
    );
    result
}

#[cfg(test)]
mod tests {
    use super::cli_help_text;

    #[test]
    fn cli_help_lists_non_interactive_flags_and_tui_exit() {
        let help = cli_help_text();
        assert!(help.contains("--smoke"));
        assert!(help.contains("--version"));
        assert!(help.contains("Ctrl+C"));
    }
}

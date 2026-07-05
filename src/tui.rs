use crate::{
    ai::{
        ModelCatalog, append_problem_note, available_models, provider_status, read_problem_notes,
        run_ai_generate, run_ai_next, run_ai_prompt,
    },
    core::{
        AI_PROVIDERS, AppState, DIFFICULTIES, HistoryItem, LANGUAGES, PROBLEM_NOTES_PATH, Problem,
        STATE_PATH, THEMES, UI_LANGUAGES, ensure_problem_files, ensure_submission, ext_for,
        give_up, judge, load_bank, load_state, localized, next_problem, normalize_ai_provider,
        normalize_difficulty, normalize_language, normalize_next_source, normalize_ui_language,
        parse_language_list, parse_topic_list, parse_ui_language_list, previous_problem,
        problem_by_id, record_pass, save_state, template_for, ui_text,
    },
    text::{
        byte_index, char_len, compose_hangul_jamo, display_width, prefix, render_markdown_plain,
    },
    update::{CURRENT_VERSION, UpdateCheck, check_latest_version},
};
use anyhow::Result;
use crossterm::event::{
    self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyEventKind,
    KeyModifiers, MouseButton, MouseEvent, MouseEventKind,
};
use crossterm::execute;
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout, Position, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
};
use std::{
    collections::HashMap,
    fs,
    io::stdout,
    path::PathBuf,
    sync::mpsc::{self, Receiver},
    thread,
    time::{Duration, Instant},
};

mod commands;
mod editor;
mod problem_view;
mod settings_panel;
use self::commands::COMMAND_HINTS;
pub use self::editor::TextEditor;

const UPDATE_CHECK_INTERVAL: Duration = Duration::from_secs(30 * 60);

#[derive(Clone)]
struct CommandChoice {
    insert: String,
    display: String,
    desc_key: &'static str,
    keep_open: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Focus {
    Code,
    Command,
    Output,
    None,
}

pub struct PracticodeApp {
    root: PathBuf,
    bank: Vec<Problem>,
    state: AppState,
    problem: Problem,
    editor: TextEditor,
    command: String,
    command_cursor: usize,
    command_palette_cursor: usize,
    output: String,
    output_is_markdown: bool,
    showing_model_status: bool,
    show_output: bool,
    focus: Focus,
    list_cursor: Option<usize>,
    settings_cursor: Option<usize>,
    busy_label: String,
    busy_body: String,
    busy_started: Option<Instant>,
    busy_frame: usize,
    busy_hits: usize,
    busy_misses: usize,
    task_rx: Option<Receiver<TaskResult>>,
    generate_rx: Option<Receiver<String>>,
    generate_bank_len: usize,
    generate_started: Option<Instant>,
    generate_notice: Option<String>,
    update_rx: Option<Receiver<UpdateCheck>>,
    model_rx: Option<Receiver<ModelCatalog>>,
    available_models: Vec<String>,
    available_models_provider: String,
    model_message: Option<String>,
    update_check: Option<UpdateCheck>,
    update_notice: Option<String>,
    last_update_check: Option<Instant>,
    code_area: Rect,
    output_area: Rect,
    command_area: Rect,
    mouse_capture: bool,
    should_quit: bool,
}

enum TaskResult {
    AiPrompt(String),
    Next {
        output: String,
        old_problem: String,
        fallback_to_local: bool,
    },
}

impl PracticodeApp {
    pub fn new(root: PathBuf) -> Result<Self> {
        let first_run = !root.join(STATE_PATH).exists();
        let bank = load_bank(&root)?;
        let state = load_state(&root, &bank)?;
        let problem = problem_by_id(&bank, &state.current_problem)
            .cloned()
            .unwrap_or_else(|| bank[0].clone());
        let mut app = Self {
            root,
            bank,
            state,
            problem,
            editor: TextEditor::default(),
            command: String::new(),
            command_cursor: 0,
            command_palette_cursor: 0,
            output: String::new(),
            output_is_markdown: false,
            showing_model_status: false,
            show_output: false,
            focus: Focus::Code,
            list_cursor: None,
            settings_cursor: None,
            busy_label: String::new(),
            busy_body: String::new(),
            busy_started: None,
            busy_frame: 0,
            busy_hits: 0,
            busy_misses: 0,
            task_rx: None,
            generate_rx: None,
            generate_bank_len: 0,
            generate_started: None,
            generate_notice: None,
            update_rx: None,
            model_rx: None,
            available_models: Vec::new(),
            available_models_provider: String::new(),
            model_message: None,
            update_check: None,
            update_notice: None,
            last_update_check: None,
            code_area: Rect::default(),
            output_area: Rect::default(),
            command_area: Rect::default(),
            mouse_capture: false,
            should_quit: false,
        };
        app.load_code_editor()?;
        if first_run {
            save_state(&app.root, &app.state)?;
            app.show_profile_with_intro(
                "Welcome to practicode\n\nUse the setup panel below first.",
            );
        }
        Ok(app)
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        self.start_update_check();
        self.start_model_check();
        while !self.should_quit {
            self.sync_mouse_capture();
            terminal.draw(|frame| self.draw(frame))?;
            self.check_task();
            self.check_background_generation();
            self.check_update();
            self.maybe_start_periodic_update_check();
            self.start_model_check();
            self.check_models();
            if event::poll(Duration::from_millis(100))? {
                match event::read()? {
                    Event::Key(key) if key.kind != KeyEventKind::Release => self.handle_key(key)?,
                    Event::Mouse(mouse) => self.handle_mouse(mouse)?,
                    _ => {}
                }
            }
            if !self.busy_label.is_empty() {
                self.busy_frame = (self.busy_frame + 1) % 32;
            }
        }
        self.save_code().ok();
        self.disable_mouse_capture();
        Ok(())
    }

    pub fn handle_command_for_test(&mut self, value: &str) -> Result<()> {
        self.handle_command(value)
    }

    pub fn focus_command_for_test(&mut self) {
        self.focus_command();
    }

    pub fn insert_command_char_for_test(&mut self, char: char) {
        self.insert_command_char(char);
    }

    pub fn command_text(&self) -> &str {
        &self.command
    }

    pub fn command_cursor(&self) -> usize {
        self.command_cursor
    }

    pub fn handle_key_for_test(&mut self, key: KeyEvent) -> Result<()> {
        self.handle_key(key)
    }

    pub fn handle_mouse_for_test(&mut self, mouse: MouseEvent) -> Result<()> {
        self.handle_mouse(mouse)
    }

    pub fn set_pane_areas_for_test(&mut self, code: Rect, output: Rect, command: Rect) {
        self.code_area = code;
        self.output_area = output;
        self.command_area = command;
    }

    pub fn busy_label(&self) -> &str {
        &self.busy_label
    }

    pub fn busy_attempts_for_test(&self) -> usize {
        self.busy_hits + self.busy_misses
    }

    pub fn has_task(&self) -> bool {
        self.task_rx.is_some()
    }

    pub fn has_background_generation_for_test(&self) -> bool {
        self.generate_rx.is_some()
    }

    pub fn check_background_generation_for_test(&mut self) {
        self.check_background_generation();
    }

    pub fn should_quit_for_test(&self) -> bool {
        self.should_quit
    }

    pub fn status_text_for_test(&self) -> String {
        self.status_text()
    }

    pub fn wants_mouse_capture_for_test(&self) -> bool {
        self.wants_mouse_capture()
    }

    pub fn output_for_test(&self) -> &str {
        &self.output
    }

    pub fn command_suggestions_for_test(&self) -> Vec<String> {
        self.command_suggestions()
            .into_iter()
            .map(|choice| choice.display)
            .collect()
    }

    pub fn set_available_models_for_test(&mut self, models: Vec<&str>) {
        self.available_models = models.into_iter().map(str::to_string).collect();
        self.available_models_provider = self.state.settings.ai_provider.clone();
        self.model_message = None;
    }

    pub fn set_model_message_for_test(&mut self, message: &str) {
        self.available_models.clear();
        self.available_models_provider = self.state.settings.ai_provider.clone();
        self.model_message = Some(message.to_string());
    }

    pub fn pane_title_for_test(title: &str, active: bool) -> String {
        Self::pane_title(title, active)
    }

    fn draw(&mut self, frame: &mut Frame) {
        let size = frame.area();
        let vertical = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(1),
                Constraint::Length(1),
                Constraint::Length(3),
            ])
            .split(size);
        let body = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(58), Constraint::Percentage(42)])
            .split(vertical[0]);
        self.code_area = body[1];
        self.output_area = body[1];
        self.command_area = vertical[2];

        let light = self.state.settings.theme == "light";
        let problem = Paragraph::new(problem_view::render(
            &self.problem,
            &self.state.settings.ui_language,
            light,
        ))
        .style(Self::pane_style(light))
        .block(Self::block(
            ui_text(&self.state.settings.ui_language, "problem"),
            light,
            false,
        ))
        .wrap(Wrap { trim: false });
        frame.render_widget(problem, body[0]);

        if self.show_output {
            let text = self.output_text();
            let output = Paragraph::new(text)
                .style(Self::pane_style(light))
                .block(Self::block(
                    ui_text(&self.state.settings.ui_language, "output"),
                    light,
                    self.focus != Focus::Command,
                ))
                .wrap(Wrap { trim: false });
            frame.render_widget(output, body[1]);
        } else {
            let code = self
                .editor
                .visible_text(body[1].height.saturating_sub(2) as usize);
            let title = format!("solution.{}", ext_for(&self.state.settings.language));
            let code = Paragraph::new(code)
                .style(Self::pane_style(light))
                .block(Self::block(&title, light, self.focus == Focus::Code));
            frame.render_widget(code, body[1]);
        }

        let status = Paragraph::new(self.status_text()).style(if light {
            Style::default()
                .fg(Color::Blue)
                .bg(Color::Rgb(219, 234, 254))
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
                .fg(Color::Rgb(200, 211, 245))
                .bg(Color::Rgb(21, 32, 51))
                .add_modifier(Modifier::BOLD)
        });
        frame.render_widget(status, vertical[1]);

        let command_text = if self.focus == Focus::Command || !self.command.is_empty() {
            self.command.clone()
        } else {
            ui_text(&self.state.settings.ui_language, "command_placeholder").to_string()
        };
        let command = Paragraph::new(command_text)
            .style(Self::pane_style(light))
            .block(Self::block(
                ui_text(&self.state.settings.ui_language, "command"),
                light,
                self.focus == Focus::Command,
            ))
            .wrap(Wrap { trim: false });
        frame.render_widget(command, vertical[2]);
        self.draw_command_palette(frame, vertical[2]);
        self.set_terminal_cursor(frame, body[1], vertical[2]);
    }

    fn wants_mouse_capture(&self) -> bool {
        !self.show_output
    }

    fn sync_mouse_capture(&mut self) {
        let want = self.wants_mouse_capture();
        if want == self.mouse_capture {
            return;
        }
        let result = if want {
            execute!(stdout(), EnableMouseCapture)
        } else {
            execute!(stdout(), DisableMouseCapture)
        };
        if result.is_ok() {
            self.mouse_capture = want;
        }
    }

    fn disable_mouse_capture(&mut self) {
        if self.mouse_capture {
            let _ = execute!(stdout(), DisableMouseCapture);
            self.mouse_capture = false;
        }
    }

    fn output_text(&self) -> Text<'static> {
        let light = self.state.settings.theme == "light";
        let title_style = if light {
            Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        };
        let label_style = if light {
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        };
        let body_style = if light {
            Style::default().fg(Color::Black)
        } else {
            Style::default().fg(Color::Rgb(229, 231, 235))
        };
        let code_style = if light {
            Style::default()
                .fg(Color::Black)
                .bg(Color::Rgb(229, 231, 235))
        } else {
            Style::default()
                .fg(Color::Rgb(243, 244, 246))
                .bg(Color::Rgb(31, 41, 55))
        };
        if !self.busy_label.is_empty() {
            let elapsed = self
                .busy_started
                .map(|started| started.elapsed().as_secs())
                .unwrap_or_default();
            let mut lines = vec![Line::from(Span::styled(
                format!("{}{}  {}s", self.busy_body, self.busy_dots(), elapsed),
                title_style,
            ))];
            if self.busy_label == "next" {
                lines.extend([
                    Line::default(),
                    Line::from(Span::styled(self.busy_game_track(), code_style)),
                    Line::from(Span::styled(
                        ui_text(&self.state.settings.ui_language, "busy_warmup").to_string(),
                        body_style,
                    )),
                    Line::from(Span::styled(
                        format!(
                            "{}: {}    {}: {}",
                            ui_text(&self.state.settings.ui_language, "hits"),
                            self.busy_hits,
                            ui_text(&self.state.settings.ui_language, "misses"),
                            self.busy_misses
                        ),
                        label_style,
                    )),
                    Line::from(Span::styled(
                        ui_text(&self.state.settings.ui_language, "busy_commands_paused")
                            .to_string(),
                        body_style,
                    )),
                ]);
            }
            return Text::from(lines);
        }
        let output = if self.output_is_markdown {
            render_markdown_plain(&self.output)
        } else {
            self.output.clone()
        };
        let mut lines = Vec::new();
        for line in output.lines() {
            if line.is_empty() {
                lines.push(Line::default());
            } else if line.starts_with("PASS ")
                || line.starts_with("FAIL ")
                || line.starts_with("Case ")
                || line.starts_with("Next:")
                || line.starts_with("Fix:")
            {
                lines.push(Line::from(Span::styled(line.to_string(), title_style)));
            } else if matches!(
                line,
                "Input" | "Expected" | "Got" | "Stdout" | "Stderr" | "Compile" | "Error"
            ) {
                lines.push(Line::from(Span::styled(line.to_string(), label_style)));
            } else if line.starts_with("  ") {
                lines.push(Line::from(vec![
                    Span::raw("  "),
                    Span::styled(line.trim_start().to_string(), code_style),
                ]));
            } else {
                lines.push(Line::from(Span::styled(line.to_string(), body_style)));
            }
        }
        Text::from(lines)
    }

    fn draw_command_palette(&self, frame: &mut Frame, command_area: Rect) {
        let suggestions = self.command_suggestions();
        if suggestions.is_empty() || command_area.y < 3 {
            return;
        }
        let height = ((suggestions.len() + 3) as u16).min(14).min(command_area.y);
        let area = Rect::new(
            command_area.x,
            command_area.y - height,
            command_area.width,
            height,
        );
        let selected = self.command_palette_cursor.min(suggestions.len() - 1);
        let visible = height.saturating_sub(2) as usize;
        let start = selected.saturating_sub(visible.saturating_sub(1));
        let mut lines = suggestions
            .iter()
            .enumerate()
            .skip(start)
            .take(visible)
            .map(|(index, hint)| {
                let marker = if index == selected { ">" } else { " " };
                format!(
                    "{marker} {:<16} {}",
                    hint.display,
                    ui_text(&self.state.settings.ui_language, hint.desc_key)
                )
            })
            .collect::<Vec<_>>();
        lines.push(ui_text(&self.state.settings.ui_language, "palette_hint").to_string());
        frame.render_widget(Clear, area);
        let light = self.state.settings.theme == "light";
        frame.render_widget(
            Paragraph::new(lines.join("\n"))
                .style(Self::pane_style(light))
                .block(Self::block(
                    ui_text(&self.state.settings.ui_language, "commands"),
                    light,
                    true,
                )),
            area,
        );
    }

    fn block(title: &str, light: bool, active: bool) -> Block<'static> {
        let border = if active {
            if light {
                Style::default()
                    .fg(Color::Magenta)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            }
        } else if light {
            Style::default().fg(Color::Blue)
        } else {
            Style::default().fg(Color::Cyan)
        };
        let border = border.bg(Self::pane_bg(light));
        Block::default()
            .borders(Borders::ALL)
            .title(Self::pane_title(title, active))
            .style(Self::pane_style(light))
            .border_style(border)
    }

    fn pane_style(light: bool) -> Style {
        if light {
            Style::default()
                .fg(Color::Rgb(17, 24, 39))
                .bg(Self::pane_bg(light))
        } else {
            Style::default()
                .fg(Color::Rgb(229, 231, 235))
                .bg(Self::pane_bg(light))
        }
    }

    fn pane_bg(light: bool) -> Color {
        if light {
            Color::Rgb(248, 250, 252)
        } else {
            Color::Rgb(17, 24, 39)
        }
    }

    pub fn pane_style_for_test(light: bool) -> Style {
        Self::pane_style(light)
    }

    fn pane_title(title: &str, active: bool) -> String {
        if active {
            format!("> {title}")
        } else {
            title.to_string()
        }
    }

    fn handle_key(&mut self, key: KeyEvent) -> Result<()> {
        if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
            self.should_quit = true;
            return Ok(());
        }
        if self.handle_busy_key(key) {
            return Ok(());
        }
        match self.focus {
            Focus::Command => self.handle_command_key(key),
            Focus::Code => self.handle_code_key(key),
            _ => self.handle_global_key(key),
        }
    }

    fn handle_mouse(&mut self, mouse: MouseEvent) -> Result<()> {
        if self.task_rx.is_some() {
            self.focus = Focus::Output;
            return Ok(());
        }
        if !matches!(mouse.kind, MouseEventKind::Down(MouseButton::Left)) {
            return Ok(());
        }
        let position = Position::new(mouse.column, mouse.row);
        if self.command_area.contains(position) {
            self.focus_command();
        } else if self.show_output && self.output_area.contains(position) {
            self.focus = Focus::Output;
        } else if self.code_area.contains(position) {
            self.action_edit()?;
        }
        Ok(())
    }

    fn handle_command_key(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Esc => {
                self.command.clear();
                self.command_cursor = 0;
                self.command_palette_cursor = 0;
                self.focus = Focus::None;
            }
            KeyCode::Enter => {
                if !self.accept_command_palette()? {
                    let value = self.command.trim().to_string();
                    self.command.clear();
                    self.command_cursor = 0;
                    self.command_palette_cursor = 0;
                    self.focus = Focus::None;
                    self.submit_command(&value)?;
                }
            }
            KeyCode::Backspace => self.delete_command_before_cursor(),
            KeyCode::Delete => self.delete_command_at_cursor(),
            KeyCode::Left => self.command_cursor = self.command_cursor.saturating_sub(1),
            KeyCode::Right => {
                self.command_cursor = (self.command_cursor + 1).min(char_len(&self.command));
            }
            KeyCode::Up => self.move_command_palette(-1),
            KeyCode::Down => self.move_command_palette(1),
            KeyCode::Home => self.command_cursor = 0,
            KeyCode::End => self.command_cursor = char_len(&self.command),
            KeyCode::Char('?') if self.command.trim().is_empty() || self.command.trim() == "/" => {
                self.command.clear();
                self.command_cursor = 0;
                self.command_palette_cursor = 0;
                self.focus = Focus::None;
                self.handle_command("help")?;
            }
            KeyCode::Char(char) if !key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.insert_command_char(char);
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_code_key(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Esc => self.focus = Focus::None,
            KeyCode::Char(char) if !key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.editor.insert_char(char);
                self.save_code()?;
            }
            KeyCode::Enter => {
                self.editor.insert_newline();
                self.save_code()?;
            }
            KeyCode::Backspace => {
                self.editor.backspace();
                self.save_code()?;
            }
            KeyCode::Delete => {
                self.editor.delete();
                self.save_code()?;
            }
            KeyCode::Tab => {
                for _ in 0..4 {
                    self.editor.insert_char(' ');
                }
                self.save_code()?;
            }
            KeyCode::Left => self.editor.move_left(),
            KeyCode::Right => self.editor.move_right(),
            KeyCode::Up => self.editor.move_up(),
            KeyCode::Down => self.editor.move_down(),
            _ => {}
        }
        Ok(())
    }

    fn handle_global_key(&mut self, key: KeyEvent) -> Result<()> {
        if self.settings_cursor.is_some() {
            match key.code {
                KeyCode::Up | KeyCode::Char('k') => self.move_settings_cursor(-1),
                KeyCode::Down | KeyCode::Char('j') => self.move_settings_cursor(1),
                KeyCode::Char(' ') | KeyCode::Enter => self.change_selected_setting()?,
                KeyCode::Esc => {
                    self.settings_cursor = None;
                    self.show_output = false;
                    self.focus = Focus::Code;
                }
                _ => self.handle_global_shortcut(key)?,
            }
            return Ok(());
        }
        if let Some(cursor) = self.list_cursor {
            match key.code {
                KeyCode::Up | KeyCode::Char('k') => self.move_list_cursor(-1),
                KeyCode::Down | KeyCode::Char('j') => self.move_list_cursor(1),
                KeyCode::Enter => self.open_selected_problem()?,
                KeyCode::Esc => {
                    self.list_cursor = None;
                    self.write_text_output("Closed list.");
                }
                _ => {
                    self.list_cursor = Some(cursor);
                    self.handle_global_shortcut(key)?;
                }
            }
            return Ok(());
        }
        if key.code == KeyCode::Esc && self.show_output {
            self.show_output = false;
            self.focus = Focus::Code;
            return Ok(());
        }
        self.handle_global_shortcut(key)
    }

    fn handle_global_shortcut(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Char('/') => self.focus_command(),
            KeyCode::Char('?') => self.handle_command("help")?,
            KeyCode::Char('r') => self.action_run()?,
            KeyCode::Char('n') => self.action_next("")?,
            KeyCode::Char('p') => self.action_previous()?,
            KeyCode::Char('g') => self.action_give_up()?,
            KeyCode::Char('e') => self.action_edit()?,
            KeyCode::Char('l') => self.action_cycle_language()?,
            KeyCode::Char('u') => self.action_toggle_ui_language()?,
            KeyCode::Char('q') => self.should_quit = true,
            _ => {}
        }
        Ok(())
    }

    fn focus_command(&mut self) {
        if self.command.is_empty() {
            self.command.push('/');
            self.command_cursor = 1;
        }
        self.command_palette_cursor = 0;
        self.focus = Focus::Command;
    }

    fn submit_command(&mut self, value: &str) -> Result<()> {
        let value = value
            .trim()
            .strip_prefix('/')
            .unwrap_or(value.trim())
            .trim();
        self.handle_command(value)
    }

    fn handle_command(&mut self, value: &str) -> Result<()> {
        if self.task_rx.is_some() {
            let command = value
                .trim()
                .strip_prefix('/')
                .unwrap_or(value.trim())
                .split_whitespace()
                .next()
                .unwrap_or("");
            if matches!(command, "exit" | "quit" | "q") {
                self.should_quit = true;
            } else {
                self.focus = Focus::Output;
            }
            return Ok(());
        }
        if value.is_empty() || matches!(value, "help" | "h" | "?") {
            self.list_cursor = None;
            self.write_output(&self.help_text());
            return Ok(());
        }
        if value.starts_with("vim") {
            self.list_cursor = None;
            self.write_text_output("The code editor is already open on the right.");
            return Ok(());
        }
        let (command, arg) = value.split_once(char::is_whitespace).unwrap_or((value, ""));
        let arg = arg.trim();
        if !matches!(command, "list" | "problems") {
            self.list_cursor = None;
        }
        match command {
            "run" | "r" => self.action_run()?,
            "code" | "edit" | "e" => self.action_edit()?,
            "next" | "n" => self.action_next(arg)?,
            "generate" | "gen" | "new" => self.action_generate(arg),
            "back" | "prev" | "previous" | "p" => self.action_previous()?,
            "answer" | "giveup" | "give" | "g" => self.action_give_up()?,
            "problems" | "list" => self.start_problem_list(),
            "open" | "o" if !arg.is_empty() => self.open_problem(arg)?,
            "language" | "lang" if arg.is_empty() => self.action_cycle_language()?,
            "language" | "lang" if LANGUAGES.contains(&arg) => self.set_language(arg)?,
            "ui" if arg.is_empty() => self.action_toggle_ui_language()?,
            "ui" => self.set_ui_language(&normalize_ui_language(arg))?,
            "theme" if arg.is_empty() => self.action_toggle_theme()?,
            "theme" if THEMES.contains(&arg) => self.set_theme(arg)?,
            "profile" | "settings" if arg.is_empty() => self.show_profile(),
            "profile" | "settings" if arg == "reset" => self.reset_profile()?,
            "difficulty" | "level" if arg.is_empty() => self.show_profile(),
            "difficulty" | "level" => self.set_difficulty(arg)?,
            "topics" | "topic" if arg.is_empty() => self.show_profile(),
            "topics" | "topic" => self.set_topics(arg, false)?,
            "avoid" | "skip" if arg.is_empty() => self.show_profile(),
            "avoid" | "skip" => self.set_topics(arg, true)?,
            "generate-languages" | "gen-languages" | "gen-lang" if arg.is_empty() => {
                self.show_profile()
            }
            "generate-languages" | "gen-languages" | "gen-lang" => {
                self.set_generate_languages(arg, false)?
            }
            "generate-ui" | "gen-ui" if arg.is_empty() => self.show_profile(),
            "generate-ui" | "gen-ui" => self.set_generate_languages(arg, true)?,
            "source" | "next-source" if arg.is_empty() => {
                self.write_text_output(&self.next_source_help());
            }
            "source" | "next-source" if matches!(arg, "bank" | "local" | "ai") => {
                self.state.settings.next_source = normalize_next_source(arg);
                save_state(&self.root, &self.state)?;
                self.write_text_output(&self.next_source_help());
            }
            "ai-next-command" if !arg.is_empty() => {
                self.state.settings.ai_next_command = arg.to_string();
                self.state.settings.next_source = "ai".to_string();
                save_state(&self.root, &self.state)?;
                self.write_text_output("AI next command saved.");
            }
            "provider" | "ai-provider" if arg.is_empty() => {
                self.write_text_output(&format!(
                    "AI provider: {}\n{}",
                    self.state.settings.ai_provider,
                    provider_status(&self.state.settings.ai_provider)
                ));
            }
            "provider" | "ai-provider" if AI_PROVIDERS.contains(&arg) => {
                self.state.settings.ai_provider = normalize_ai_provider(arg);
                self.model_rx = None;
                self.available_models.clear();
                self.available_models_provider.clear();
                self.model_message = None;
                save_state(&self.root, &self.state)?;
                self.write_text_output(&format!(
                    "AI provider: {}\n{}",
                    self.state.settings.ai_provider,
                    provider_status(&self.state.settings.ai_provider)
                ));
            }
            "model" if arg.is_empty() => {
                self.start_model_check();
                self.check_models();
                self.write_model_status();
            }
            "model" => {
                self.state.settings.ai_model = if arg == "auto" {
                    "auto".to_string()
                } else {
                    arg.to_string()
                };
                save_state(&self.root, &self.state)?;
                self.start_model_check();
                self.check_models();
                self.write_model_status();
            }
            "hint" if arg.is_empty() => {
                self.start_ai_prompt("Give one concise hint for the current problem.")?
            }
            "hint" | "ask" | "ai" if !arg.is_empty() => self.start_ai_prompt(arg)?,
            "note" if !arg.is_empty() => self.append_note(arg)?,
            "note" | "notes" => self.show_notes()?,
            "update" => self.refresh_update_notice(),
            "exit" | "quit" | "q" => self.should_quit = true,
            _ => self.write_text_output(&format!("Unknown command: {value}\nTry /help.")),
        }
        Ok(())
    }

    fn action_edit(&mut self) -> Result<()> {
        self.load_code_editor()?;
        self.settings_cursor = None;
        self.show_output = false;
        self.focus = Focus::Code;
        Ok(())
    }

    fn action_run(&mut self) -> Result<()> {
        self.save_code()?;
        let result = judge(&self.root, &self.problem, &self.state.settings);
        if result.passed {
            record_pass(&self.root, &self.problem, &mut self.state)?;
        }
        let headline = format!(
            "{} {}/{}",
            if result.passed { "PASS" } else { "FAIL" },
            result.passed_cases,
            result.total_cases
        );
        let next_step = if result.passed {
            ui_text(&self.state.settings.ui_language, "run_pass_next")
        } else {
            ui_text(&self.state.settings.ui_language, "run_fail_next")
        };
        self.write_text_output(&format!("{headline}\n{}\n\n{next_step}", result.output));
        Ok(())
    }

    fn action_next(&mut self, request: &str) -> Result<()> {
        self.check_background_generation();
        let request = request.trim();
        let old_problem = self.state.current_problem.clone();
        if let Some(problem) = next_problem(&self.root, &self.bank, &mut self.state)? {
            self.generate_notice = None;
            self.problem = problem;
            self.load_code_editor()?;
            self.settings_cursor = None;
            self.show_output = false;
            self.focus = Focus::Code;
            return Ok(());
        }
        if self.generate_rx.is_some() {
            self.write_text_output(
                "A background generation is already running. Keep solving; /next will pick up the new problem when it finishes.",
            );
            return Ok(());
        }
        self.start_next_problem(old_problem, true, request.to_string());
        Ok(())
    }

    fn action_generate(&mut self, request: &str) {
        self.check_background_generation();
        if self.task_rx.is_some() || self.generate_rx.is_some() {
            let message = "Generation is already running; skipped duplicate /generate.";
            self.generate_notice = Some(message.to_string());
            self.write_text_output(message);
            return;
        }
        self.start_background_generation(request.trim().to_string());
    }

    fn start_background_generation(&mut self, request: String) {
        let root = self.root.clone();
        let state = self.state.clone();
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let _ = tx.send(run_ai_generate(&root, &state, &request));
        });
        self.generate_bank_len = self.bank.len();
        self.generate_started = Some(Instant::now());
        self.generate_notice = Some("Generating in background.".to_string());
        self.generate_rx = Some(rx);
        self.settings_cursor = None;
        self.show_output = false;
        self.focus = Focus::Code;
    }

    fn start_next_problem(
        &mut self,
        old_problem: String,
        fallback_to_local: bool,
        request: String,
    ) {
        if self.task_rx.is_some() {
            self.write_text_output(ui_text(&self.state.settings.ui_language, "already_busy"));
            return;
        }
        self.start_busy(
            "next",
            ui_text(&self.state.settings.ui_language, "generating_next"),
        );
        let root = self.root.clone();
        let state = self.state.clone();
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let output = run_ai_next(&root, &state, true, &request);
            let _ = tx.send(TaskResult::Next {
                output,
                old_problem,
                fallback_to_local,
            });
        });
        self.task_rx = Some(rx);
    }

    fn finish_next_problem(
        &mut self,
        output: String,
        old_problem: String,
        fallback_to_local: bool,
    ) -> Result<()> {
        self.bank = load_bank(&self.root)?;
        self.state = load_state(&self.root, &self.bank)?;
        self.problem = problem_by_id(&self.bank, &self.state.current_problem)
            .cloned()
            .unwrap_or_else(|| self.bank[0].clone());
        if self.state.current_problem == old_problem {
            if fallback_to_local
                && let Some(problem) = next_problem(&self.root, &self.bank, &mut self.state)?
            {
                self.problem = problem;
            } else {
                self.write_text_output(&format!(
                    "{}{}No next problem is available yet.",
                    if output.is_empty() { "" } else { &output },
                    if output.is_empty() { "" } else { "\n\n" }
                ));
                return Ok(());
            }
        }
        self.load_code_editor()?;
        self.settings_cursor = None;
        self.show_output = false;
        self.focus = Focus::Code;
        Ok(())
    }

    fn action_previous(&mut self) -> Result<()> {
        let old_problem = self.state.current_problem.clone();
        self.problem = previous_problem(&self.root, &self.bank, &mut self.state)?;
        if self.state.current_problem == old_problem {
            self.write_text_output("Already at the first known problem.");
        } else {
            self.load_code_editor()?;
            self.settings_cursor = None;
            self.show_output = false;
            self.focus = Focus::Code;
        }
        Ok(())
    }

    fn action_give_up(&mut self) -> Result<()> {
        let answer = give_up(&self.root, &self.problem, &mut self.state)?;
        let language = normalize_language(&self.state.settings.language);
        self.write_output(&format!(
            "Answer for {language}:\n\n```{language}\n{}\n```",
            answer.trim_end()
        ));
        Ok(())
    }

    fn action_cycle_language(&mut self) -> Result<()> {
        let current = LANGUAGES
            .iter()
            .position(|language| language == &self.state.settings.language)
            .unwrap_or(0);
        self.set_language(LANGUAGES[(current + 1) % LANGUAGES.len()])
    }

    fn action_toggle_ui_language(&mut self) -> Result<()> {
        let current = UI_LANGUAGES
            .iter()
            .position(|language| language == &self.state.settings.ui_language)
            .unwrap_or(0);
        self.set_ui_language(UI_LANGUAGES[(current + 1) % UI_LANGUAGES.len()])
    }

    fn action_toggle_theme(&mut self) -> Result<()> {
        let current = THEMES
            .iter()
            .position(|theme| theme == &self.state.settings.theme)
            .unwrap_or(0);
        self.set_theme(THEMES[(current + 1) % THEMES.len()])
    }

    fn set_language(&mut self, language: &str) -> Result<()> {
        self.state.settings.language = language.to_string();
        save_state(&self.root, &self.state)?;
        self.load_code_editor()?;
        self.settings_cursor = None;
        self.show_output = false;
        self.focus = Focus::Code;
        Ok(())
    }

    fn set_ui_language(&mut self, language: &str) -> Result<()> {
        self.state.settings.ui_language = normalize_ui_language(language);
        save_state(&self.root, &self.state)?;
        self.write_text_output(&format!("UI language: {}", self.state.settings.ui_language));
        Ok(())
    }

    fn set_theme(&mut self, theme: &str) -> Result<()> {
        self.state.settings.theme = theme.to_string();
        save_state(&self.root, &self.state)?;
        self.write_text_output(&format!("Theme: {theme}"));
        Ok(())
    }

    fn set_difficulty(&mut self, difficulty: &str) -> Result<()> {
        let difficulty = difficulty.trim().to_lowercase();
        if !DIFFICULTIES.contains(&difficulty.as_str()) {
            self.write_text_output("Difficulty: auto, easy, medium, or hard.");
            return Ok(());
        }
        let normalized = normalize_difficulty(&difficulty);
        self.state.settings.difficulty = normalized.clone();
        if normalized != "auto" {
            self.state.suggested_next_difficulty = normalized;
        }
        save_state(&self.root, &self.state)?;
        self.show_profile();
        Ok(())
    }

    fn set_topics(&mut self, topics: &str, avoid: bool) -> Result<()> {
        let topics = parse_topic_list(topics);
        if avoid {
            self.state.settings.avoid_topics = topics;
        } else {
            self.state.settings.topics = topics;
        }
        save_state(&self.root, &self.state)?;
        self.show_profile();
        Ok(())
    }

    fn set_generate_languages(&mut self, value: &str, ui: bool) -> Result<()> {
        if ui {
            self.state.settings.generate_ui_languages = parse_ui_language_list(value);
        } else {
            self.state.settings.generate_languages = parse_language_list(value);
        }
        save_state(&self.root, &self.state)?;
        self.show_profile();
        Ok(())
    }

    fn reset_profile(&mut self) -> Result<()> {
        self.state.settings.difficulty = "auto".to_string();
        self.state.settings.topics.clear();
        self.state.settings.avoid_topics.clear();
        self.state.settings.generate_languages.clear();
        self.state.settings.generate_ui_languages.clear();
        save_state(&self.root, &self.state)?;
        self.show_profile();
        Ok(())
    }

    fn show_profile(&mut self) {
        self.show_profile_with_intro("");
    }

    fn show_profile_with_intro(&mut self, intro: &str) {
        self.showing_model_status = false;
        if self.settings_cursor.is_none() {
            self.settings_cursor = Some(0);
        }
        let profile = self.profile_text();
        self.output = if intro.trim().is_empty() {
            profile
        } else {
            format!("{}\n\n{profile}", intro.trim_end())
        };
        self.output_is_markdown = false;
        self.show_output = true;
        self.focus = Focus::Output;
    }

    fn profile_text(&self) -> String {
        settings_panel::render(&self.state, self.settings_cursor)
    }

    fn settings_row_count(&self) -> usize {
        settings_panel::row_count()
    }

    fn move_settings_cursor(&mut self, delta: isize) {
        let len = self.settings_row_count() as isize;
        let cursor = self.settings_cursor.unwrap_or(0) as isize;
        self.settings_cursor = Some(((cursor + delta).rem_euclid(len)) as usize);
        self.show_profile();
    }

    fn change_selected_setting(&mut self) -> Result<()> {
        let Some(row) = self.settings_cursor else {
            return Ok(());
        };
        let change = settings_panel::apply_selected(&mut self.state, row);
        if change.reload_editor {
            self.load_code_editor()?;
        }
        save_state(&self.root, &self.state)?;
        self.show_profile();
        Ok(())
    }

    fn start_ai_prompt(&mut self, prompt: &str) -> Result<()> {
        if self.task_rx.is_some() {
            self.write_text_output(ui_text(&self.state.settings.ui_language, "already_busy"));
            return Ok(());
        }
        self.save_code()?;
        let label = normalize_ai_provider(&self.state.settings.ai_provider);
        self.start_busy("ai", &format!("{label} is thinking"));
        let root = self.root.clone();
        let problem = self.problem.clone();
        let settings = self.state.settings.clone();
        let prompt = prompt.to_string();
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let output = run_ai_prompt(&root, &problem, &settings, &prompt);
            let _ = tx.send(TaskResult::AiPrompt(output));
        });
        self.task_rx = Some(rx);
        Ok(())
    }

    fn check_task(&mut self) {
        let task = self.task_rx.as_ref().and_then(|rx| rx.try_recv().ok());
        if let Some(task) = task {
            self.task_rx = None;
            self.stop_busy();
            match task {
                TaskResult::AiPrompt(output) => self.write_output(&output),
                TaskResult::Next {
                    output,
                    old_problem,
                    fallback_to_local,
                } => {
                    if let Err(error) =
                        self.finish_next_problem(output, old_problem, fallback_to_local)
                    {
                        self.write_text_output(&format!("Next failed\n{error}"));
                    }
                }
            }
        }
    }

    fn check_background_generation(&mut self) {
        let output = self.generate_rx.as_ref().and_then(|rx| rx.try_recv().ok());
        let Some(output) = output else {
            return;
        };
        self.generate_rx = None;
        self.generate_started = None;
        let old_len = self.generate_bank_len;
        match load_bank(&self.root) {
            Ok(bank) => {
                let added = bank.len().saturating_sub(old_len);
                self.bank = bank;
                let _ = save_state(&self.root, &self.state);
                self.generate_notice = Some(if added > 0 {
                    format!("Generated {added} problem in background. Use /next.")
                } else if output.contains("failed") {
                    "Background generation failed. Use /generate to retry.".to_string()
                } else {
                    "Background generation finished. Use /problems to review.".to_string()
                });
            }
            Err(error) => {
                self.generate_notice = Some(format!(
                    "Background generation finished, but bank reload failed: {error}"
                ));
            }
        }
    }

    fn check_update(&mut self) {
        let result = self.update_rx.as_ref().and_then(|rx| rx.try_recv().ok());
        if let Some(result) = result {
            self.update_rx = None;
            self.update_check = Some(result.clone());
            match &result {
                UpdateCheck::Available(version) => self.update_notice = Some(version.clone()),
                UpdateCheck::Current | UpdateCheck::Disabled => self.update_notice = None,
                UpdateCheck::Failed => {}
            }
        }
    }

    fn start_update_check(&mut self) {
        if self.update_rx.is_some() {
            return;
        }
        self.last_update_check = Some(Instant::now());
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let _ = tx.send(check_latest_version());
        });
        self.update_rx = Some(rx);
    }

    fn maybe_start_periodic_update_check(&mut self) {
        if self.update_rx.is_some() {
            return;
        }
        if self
            .last_update_check
            .is_none_or(|last| last.elapsed() >= UPDATE_CHECK_INTERVAL)
        {
            self.start_update_check();
        }
    }

    fn start_model_check(&mut self) {
        let provider = self.state.settings.ai_provider.clone();
        if self.model_rx.is_some() || self.available_models_provider == provider {
            return;
        }
        let query_provider = provider.clone();
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let _ = tx.send(available_models(&query_provider));
        });
        self.available_models_provider = provider;
        self.model_rx = Some(rx);
    }

    fn check_models(&mut self) {
        let models = self.model_rx.as_ref().and_then(|rx| rx.try_recv().ok());
        if let Some(catalog) = models {
            self.model_rx = None;
            self.available_models = catalog.models;
            self.model_message = catalog.message;
            if self.showing_model_status {
                self.output = self.model_status_text();
                self.output_is_markdown = false;
                self.show_output = true;
            }
        }
    }

    fn model_status_text(&self) -> String {
        let mut lines = vec![
            format!("AI provider: {}", self.state.settings.ai_provider),
            format!(
                "AI model: {}",
                if self.state.settings.ai_model == "auto" {
                    "auto (provider default)"
                } else {
                    self.state.settings.ai_model.as_str()
                }
            ),
            "Use /model auto to let the provider choose its default.".to_string(),
        ];
        if self.model_rx.is_some() {
            lines.push("Loading provider model list...".to_string());
        } else if self.available_models.is_empty() {
            lines.push(
                self.model_message
                    .clone()
                    .unwrap_or_else(|| "Provider model list is unavailable.".to_string()),
            );
            lines.push("Use /model <name> for a known model.".to_string());
        } else {
            lines.push("Available models:".to_string());
            lines.extend(
                self.available_models
                    .iter()
                    .map(|model| format!("- /model {model}")),
            );
        }
        lines.join("\n")
    }

    fn start_busy(&mut self, label: &str, body: &str) {
        self.settings_cursor = None;
        self.busy_label = label.to_string();
        self.busy_body = body.to_string();
        self.busy_started = Some(Instant::now());
        self.busy_frame = 0;
        self.busy_hits = 0;
        self.busy_misses = 0;
        self.show_output = true;
        self.focus = Focus::Output;
    }

    fn stop_busy(&mut self) {
        self.busy_label.clear();
        self.busy_body.clear();
        self.busy_started = None;
        self.busy_frame = 0;
    }

    fn handle_busy_key(&mut self, key: KeyEvent) -> bool {
        if self.task_rx.is_none() {
            return false;
        }
        if key.code == KeyCode::Char('q') && key.modifiers.is_empty() {
            self.should_quit = true;
        } else if self.busy_label == "next"
            && key.code == KeyCode::Char(' ')
            && key.modifiers.is_empty()
        {
            if self.busy_game_on_target() {
                self.busy_hits += 1;
            } else {
                self.busy_misses += 1;
            }
        }
        self.focus = Focus::Output;
        true
    }

    fn write_output(&mut self, output: &str) {
        self.settings_cursor = None;
        self.showing_model_status = false;
        self.output = output.to_string();
        self.output_is_markdown = true;
        self.show_output = true;
        self.focus = Focus::Output;
    }

    fn write_text_output(&mut self, output: &str) {
        self.settings_cursor = None;
        self.showing_model_status = false;
        self.output = output.trim_end().to_string();
        self.output_is_markdown = false;
        self.show_output = true;
        self.focus = Focus::Output;
    }

    fn write_model_status(&mut self) {
        self.output = self.model_status_text();
        self.output_is_markdown = false;
        self.showing_model_status = true;
        self.show_output = true;
        self.focus = Focus::Output;
    }

    fn refresh_update_notice(&mut self) {
        self.update_check = None;
        self.update_notice = None;
        self.start_update_check();
        self.show_update_notice();
    }

    fn show_update_notice(&mut self) {
        let lang = self.state.settings.ui_language.clone();
        if let Some(version) = &self.update_notice {
            self.write_text_output(&format!(
                "{}: practicode {version} (current {CURRENT_VERSION})\n\nnpm update -g practicode\ncargo install --force practicode",
                ui_text(&lang, "update_available")
            ));
        } else if self.update_rx.is_some() {
            self.write_text_output("Checking for updates...");
        } else if matches!(self.update_check, Some(UpdateCheck::Disabled)) {
            self.write_text_output(ui_text(&lang, "update_check_disabled"));
        } else if matches!(self.update_check, Some(UpdateCheck::Failed)) {
            self.write_text_output(ui_text(&lang, "update_check_failed"));
        } else {
            self.write_text_output(ui_text(&lang, "update_none"));
        }
    }

    fn append_note(&mut self, note: &str) -> Result<()> {
        append_problem_note(&self.root, note)?;
        self.write_text_output(&format!("Problem note saved to {PROBLEM_NOTES_PATH}."));
        Ok(())
    }

    fn show_notes(&mut self) -> Result<()> {
        let notes = read_problem_notes(&self.root)?;
        if notes.is_empty() {
            self.write_text_output("No notes yet. Use /topics or /avoid for standing preferences.");
        } else {
            self.write_text_output(&format!("Problem notes ({PROBLEM_NOTES_PATH})\n\n{notes}"));
        }
        Ok(())
    }

    fn insert_command_char(&mut self, char: char) {
        let byte = byte_index(&self.command, self.command_cursor);
        self.command.insert(byte, char);
        self.command_cursor += 1;
        self.command_palette_cursor = 0;
        self.normalize_command_input();
    }

    fn delete_command_before_cursor(&mut self) {
        if self.command_cursor == 0 {
            return;
        }
        let start = byte_index(&self.command, self.command_cursor - 1);
        let end = byte_index(&self.command, self.command_cursor);
        self.command.replace_range(start..end, "");
        self.command_cursor -= 1;
        self.command_palette_cursor = 0;
        self.normalize_command_input();
    }

    fn delete_command_at_cursor(&mut self) {
        if self.command_cursor >= char_len(&self.command) {
            return;
        }
        let start = byte_index(&self.command, self.command_cursor);
        let end = byte_index(&self.command, self.command_cursor + 1);
        self.command.replace_range(start..end, "");
        self.command_palette_cursor = 0;
        self.normalize_command_input();
    }

    fn command_suggestions(&self) -> Vec<CommandChoice> {
        if self.focus != Focus::Command {
            return Vec::new();
        }
        let Some(query) = self.command.trim_start().strip_prefix('/') else {
            return Vec::new();
        };
        let query = query.to_lowercase();
        self.command_choices()
            .into_iter()
            .filter(|hint| hint.insert.starts_with(query.trim_start()))
            .collect()
    }

    fn command_choices(&self) -> Vec<CommandChoice> {
        let mut choices = Vec::new();
        for hint in COMMAND_HINTS {
            if hint.insert == "model " {
                for model in self
                    .available_models
                    .iter()
                    .filter(|model| *model != "auto")
                {
                    choices.push(CommandChoice {
                        insert: format!("model {model}"),
                        display: format!("/model {model}"),
                        desc_key: "cmd_model_available",
                        keep_open: false,
                    });
                }
            }
            choices.push(CommandChoice {
                insert: hint.insert.to_string(),
                display: hint.display.to_string(),
                desc_key: hint.desc_key,
                keep_open: hint.keep_open,
            });
        }
        choices
    }

    fn move_command_palette(&mut self, delta: isize) {
        let len = self.command_suggestions().len();
        if len == 0 {
            return;
        }
        let cursor = self.command_palette_cursor as isize;
        self.command_palette_cursor = ((cursor + delta).rem_euclid(len as isize)) as usize;
    }

    fn accept_command_palette(&mut self) -> Result<bool> {
        let suggestions = self.command_suggestions();
        if suggestions.is_empty() {
            return Ok(false);
        }
        let hint = &suggestions[self.command_palette_cursor.min(suggestions.len() - 1)];
        if hint.keep_open {
            self.command = format!("/{}", hint.insert);
            self.command_cursor = char_len(&self.command);
            self.command_palette_cursor = 0;
            return Ok(true);
        }
        let value = hint.insert.clone();
        self.command.clear();
        self.command_cursor = 0;
        self.command_palette_cursor = 0;
        self.focus = Focus::None;
        self.submit_command(&value)?;
        Ok(true)
    }

    fn normalize_command_input(&mut self) {
        let normalized = compose_hangul_jamo(&self.command);
        if normalized == self.command {
            self.command_cursor = self.command_cursor.min(char_len(&self.command));
            return;
        }
        let old_prefix = prefix(&self.command, self.command_cursor);
        self.command = normalized;
        self.command_cursor =
            char_len(&compose_hangul_jamo(&old_prefix)).min(char_len(&self.command));
    }

    fn set_terminal_cursor(&self, frame: &mut Frame, code_area: Rect, command_area: Rect) {
        match self.focus {
            Focus::Command => {
                let before = prefix(&self.command, self.command_cursor);
                let x = command_area
                    .x
                    .saturating_add(1)
                    .saturating_add(display_width(&before) as u16)
                    .min(command_area.right().saturating_sub(2));
                frame.set_cursor_position(Position::new(x, command_area.y.saturating_add(1)));
            }
            Focus::Code if !self.show_output => {
                if let Some(position) = self.editor.cursor_position(code_area) {
                    frame.set_cursor_position(position);
                }
            }
            _ => {}
        }
    }

    fn load_code_editor(&mut self) -> Result<()> {
        let path = ensure_submission(&self.root, &self.problem, &self.state.settings)?;
        let text = fs::read_to_string(path).unwrap_or_default();
        self.editor.set_text(&text);
        Ok(())
    }

    fn save_code(&self) -> Result<()> {
        let path = ensure_submission(&self.root, &self.problem, &self.state.settings)?;
        fs::write(path, self.editor.text())?;
        Ok(())
    }

    fn start_problem_list(&mut self) {
        self.list_cursor = Some(self.current_problem_index());
        self.write_text_output(&self.render_problem_list());
    }

    fn render_problem_list(&self) -> String {
        let status_by_id = self
            .state
            .history
            .iter()
            .map(|item| (item.id.as_str(), item.status.as_str()))
            .collect::<HashMap<_, _>>();
        let cursor = self
            .list_cursor
            .unwrap_or_else(|| self.current_problem_index());
        let mut lines = vec![
            "Problems".to_string(),
            String::new(),
            "    # ID                 Difficulty  Status      Code      Title".to_string(),
        ];
        for (index, problem) in self.bank.iter().enumerate() {
            let marker = if index == cursor { ">" } else { " " };
            let current = if problem.id == self.problem.id {
                "*"
            } else {
                " "
            };
            let title = localized(&problem.title, &self.state.settings.ui_language);
            let code_status = self.submission_status(problem).0;
            lines.push(format!(
                "{marker} {current} {:>2} {:<18} {:<10} {:<10} {:<9} {title}",
                index + 1,
                problem.id,
                problem.difficulty,
                status_by_id
                    .get(problem.id.as_str())
                    .copied()
                    .unwrap_or("-"),
                code_status,
            ));
        }
        lines.push("\nup/down or j/k select | enter open | esc close".to_string());
        lines.join("\n")
    }

    fn current_problem_index(&self) -> usize {
        self.bank
            .iter()
            .position(|problem| problem.id == self.problem.id)
            .unwrap_or(0)
    }

    fn move_list_cursor(&mut self, delta: isize) {
        if self.bank.is_empty() {
            return;
        }
        let cursor = self
            .list_cursor
            .unwrap_or_else(|| self.current_problem_index()) as isize;
        let len = self.bank.len() as isize;
        self.list_cursor = Some(((cursor + delta).rem_euclid(len)) as usize);
        self.write_text_output(&self.render_problem_list());
    }

    fn open_selected_problem(&mut self) -> Result<()> {
        if let Some(cursor) = self.list_cursor {
            let problem_id = self.bank[cursor].id.clone();
            self.list_cursor = None;
            self.open_problem(&problem_id)?;
        }
        Ok(())
    }

    fn open_problem(&mut self, query: &str) -> Result<()> {
        self.list_cursor = None;
        let Some(problem) = self.find_problem(query).cloned() else {
            self.write_text_output(&format!("Problem not found: {query}\nTry /problems."));
            return Ok(());
        };
        self.problem = problem;
        self.state.current_problem = self.problem.id.clone();
        if !self
            .state
            .history
            .iter()
            .any(|item| item.id == self.problem.id)
        {
            self.state.history.push(HistoryItem {
                id: self.problem.id.clone(),
                status: "assigned".to_string(),
            });
        }
        save_state(&self.root, &self.state)?;
        ensure_problem_files(&self.root, &self.problem)?;
        self.load_code_editor()?;
        self.show_output = false;
        self.focus = Focus::Code;
        Ok(())
    }

    fn find_problem(&self, query: &str) -> Option<&Problem> {
        let needle = if query.trim().chars().all(|c| c.is_ascii_digit()) {
            format!("{:03}", query.trim().parse::<usize>().ok()?)
        } else {
            query.trim().to_lowercase()
        };
        self.bank.iter().find(|problem| {
            needle == problem.id.to_lowercase()
                || needle == problem.slug.to_lowercase()
                || problem.id.starts_with(&needle)
        })
    }

    fn problem_status(&self, problem: &Problem) -> String {
        if self.state.solved.contains(&problem.id) {
            return "solved".to_string();
        }
        self.state
            .history
            .iter()
            .rev()
            .find(|item| item.id == problem.id)
            .map(|item| item.status.clone())
            .unwrap_or_else(|| "not_started".to_string())
    }

    fn submission_status(&self, problem: &Problem) -> (String, String) {
        let language = normalize_language(&self.state.settings.language);
        let path = self
            .root
            .join("submissions")
            .join(&problem.id)
            .join(format!("solution.{}", ext_for(&language)));
        if !path.exists() {
            return ("missing".to_string(), format!("({language})"));
        }
        let content = fs::read_to_string(&path).unwrap_or_default();
        let relative = path.strip_prefix(&self.root).unwrap_or(&path).display();
        if content == template_for(&language) {
            ("template".to_string(), format!("({relative})"))
        } else if content.trim().is_empty() {
            ("empty".to_string(), format!("({relative})"))
        } else {
            ("written".to_string(), format!("({relative})"))
        }
    }

    fn status_text(&self) -> String {
        let code_status = self.submission_status(&self.problem).0;
        let activity = if self.busy_label.is_empty() {
            "idle".to_string()
        } else {
            format!("{}{}", self.busy_body, self.busy_dots())
        };
        let tail = if let Some(version) = self.update_notice.as_ref() {
            format!(
                "{}:{version} /update",
                ui_text(&self.state.settings.ui_language, "update")
            )
        } else if self.task_rx.is_some() {
            self.mode_hint().to_string()
        } else if let Some(status) = self.background_generation_status() {
            status
        } else {
            self.mode_hint().to_string()
        };
        format!(
            " PRACTICODE | {} | {} | {} | {} | code:{} | {} | {} ",
            self.problem.id,
            self.problem.difficulty,
            self.problem_status(&self.problem),
            activity,
            code_status,
            self.state.settings.language,
            tail,
        )
    }

    fn next_source_help(&self) -> String {
        "Next behavior: /next opens unsolved local problems first and asks AI only when none remain. Use /generate <request> to create a problem in the background.".to_string()
    }

    fn background_generation_status(&self) -> Option<String> {
        if self.generate_rx.is_some() {
            let elapsed = self
                .generate_started
                .map(|started| started.elapsed().as_secs())
                .unwrap_or_default();
            Some(format!("bg generate {elapsed}s"))
        } else {
            self.generate_notice.clone()
        }
    }

    fn busy_dots(&self) -> String {
        ".".repeat((self.busy_frame / 8) % 4)
    }

    fn busy_game_track(&self) -> String {
        let width = 9;
        let target = width / 2;
        let position = (self.busy_frame / 2) % width;
        let mut cells = vec!['-'; width];
        cells[target] = '|';
        cells[position] = if position == target { 'X' } else { '*' };
        format!("[{}]", cells.into_iter().collect::<String>())
    }

    fn busy_game_on_target(&self) -> bool {
        (self.busy_frame / 2) % 9 == 4
    }

    fn mode_hint(&self) -> &'static str {
        let lang = &self.state.settings.ui_language;
        if self.task_rx.is_some() {
            return if self.busy_label == "next" {
                ui_text(lang, "hint_busy_next")
            } else {
                ui_text(lang, "hint_busy")
            };
        }
        match (self.focus, self.list_cursor.is_some(), self.show_output) {
            (Focus::Command, _, _) => ui_text(lang, "hint_command"),
            (_, true, _) => ui_text(lang, "hint_list"),
            (_, _, true) if self.settings_cursor.is_some() => ui_text(lang, "hint_settings"),
            (_, _, true) => ui_text(lang, "hint_output"),
            (Focus::Code, _, _) => ui_text(lang, "hint_code"),
            _ => ui_text(lang, "hint_idle"),
        }
    }

    fn help_text(&self) -> String {
        let lang = &self.state.settings.ui_language;
        let commands = COMMAND_HINTS
            .iter()
            .filter(|hint| hint.help)
            .map(|hint| format!("- `{}` {}", hint.display, ui_text(lang, hint.desc_key)))
            .collect::<Vec<_>>()
            .join("\n");
        format!(
            "# {}\n\n## {}\n\n1. Type code in the right pane.\n2. Press `Esc`, then choose `/run` from the command palette.\n3. Use `/next` when it passes.\n\n## {}\n\n{}\n\n## {}\n\n- `/` opens the command palette outside the editor.\n- `↑/↓` selects a command and `Enter` accepts it.\n- `Esc` cancels the command palette or leaves output.\n\n## {}\n\n- stdout is shown when a case fails.\n- stderr is shown without affecting the expected stdout.",
            ui_text(lang, "help_title"),
            ui_text(lang, "daily_loop"),
            ui_text(lang, "commands"),
            commands,
            ui_text(lang, "keys"),
            ui_text(lang, "debug_prints"),
        )
    }
}

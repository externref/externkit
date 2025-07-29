use crate::editor::editor::Editor;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    execute,
    style::Print,
    terminal::{self, ClearType},
};
use std::fs;
use std::io::{self, stdout, Write};

pub struct InputHandler;

impl InputHandler {
    pub fn new() -> Self {
        InputHandler
    }

    pub fn process_key(&mut self, editor: &mut Editor, key: KeyEvent) -> io::Result<bool> {
        match key {
            KeyEvent {
                code: KeyCode::Char('x'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => {
                if editor.modified {
                    if self.prompt_save(editor)? {
                        self.save_file(editor)?;
                    }
                }
                return Ok(true);
            }
            KeyEvent {
                code: KeyCode::Char('s'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => {
                self.save_file(editor)?;
            }
            KeyEvent {
                code: KeyCode::Char('o'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => {
                if let Some(filename) = self.prompt_filename(editor)? {
                    *editor = Editor::open_file(&filename)?;
                }
            }
            KeyEvent {
                code: KeyCode::Up, ..
            } => editor.move_cursor_up(),

            KeyEvent {
                code: KeyCode::Down,
                ..
            } => editor.move_cursor_down(),

            KeyEvent {
                code: KeyCode::Left,
                ..
            } => editor.move_cursor_left(),

            KeyEvent {
                code: KeyCode::Right,
                ..
            } => editor.move_cursor_right(),

            // Home/End
            KeyEvent {
                code: KeyCode::Home,
                ..
            } => editor.cursor_x = 0,

            KeyEvent {
                code: KeyCode::End, ..
            } => {
                if editor.cursor_y < editor.content.len() {
                    editor.cursor_x = editor.content[editor.cursor_y].len();
                }
            }
            KeyEvent {
                code: KeyCode::Enter,
                ..
            } => {
                editor.insert_newline();
            }
            KeyEvent {
                code: KeyCode::Backspace,
                ..
            } => {
                editor.delete_char();
            }
            KeyEvent {
                code: KeyCode::Delete,
                ..
            } => {
                editor.delete_char_forward();
            }
            KeyEvent {
                code: KeyCode::Tab, ..
            } => {
                editor.insert_char('\t');
            }
            KeyEvent {
                code: KeyCode::Char(c),
                modifiers: KeyModifiers::NONE | KeyModifiers::SHIFT,
                ..
            } => {
                if c.is_control() {
                    return Ok(false);
                }
                editor.insert_char(c);
            }

            _ => {}
        }

        Ok(false)
    }

    fn save_file(&mut self, editor: &mut Editor) -> io::Result<()> {
        let filename = if let Some(ref name) = editor.filename {
            name.clone()
        } else {
            if let Some(name) = self.prompt_filename(editor)? {
                editor.filename = Some(name.clone());
                name
            } else {
                return Ok(());
            }
        };

        let content = editor.content.join("\n");
        fs::write(&filename, content)?;
        editor.modified = false;
        Ok(())
    }

    fn prompt_save(&self, editor: &Editor) -> io::Result<bool> {
        execute!(
            stdout(),
            cursor::MoveTo(0, editor.terminal_height as u16 - 1)
        )?;
        execute!(stdout(), terminal::Clear(ClearType::CurrentLine))?;
        execute!(stdout(), Print("Save modified buffer? (y/n): "))?;
        stdout().flush()?;

        loop {
            if let Event::Key(KeyEvent { code, kind, .. }) = event::read()? {
                // Only process Press events for prompts too
                if kind == KeyEventKind::Press {
                    match code {
                        KeyCode::Char('y') | KeyCode::Char('Y') => return Ok(true),
                        KeyCode::Char('n') | KeyCode::Char('N') => return Ok(false),
                        KeyCode::Esc => return Ok(false),
                        _ => {}
                    }
                }
            }
        }
    }

    fn prompt_filename(&self, editor: &Editor) -> io::Result<Option<String>> {
        execute!(
            stdout(),
            cursor::MoveTo(0, editor.terminal_height as u16 - 1)
        )?;
        execute!(stdout(), terminal::Clear(ClearType::CurrentLine))?;
        execute!(stdout(), Print("File name to write: "))?;
        stdout().flush()?;

        let mut filename = String::new();
        loop {
            if let Event::Key(KeyEvent { code, kind, .. }) = event::read()? {
                // Only process Press events for prompts too
                if kind == KeyEventKind::Press {
                    match code {
                        KeyCode::Enter => {
                            if !filename.is_empty() {
                                return Ok(Some(filename));
                            }
                            return Ok(None);
                        }
                        KeyCode::Esc => return Ok(None),
                        KeyCode::Backspace => {
                            if !filename.is_empty() {
                                filename.pop();
                                execute!(stdout(), cursor::MoveLeft(1))?;
                                execute!(stdout(), Print(" "))?;
                                execute!(stdout(), cursor::MoveLeft(1))?;
                                stdout().flush()?;
                            }
                        }
                        KeyCode::Char(c) if !c.is_control() => {
                            filename.push(c);
                            execute!(stdout(), Print(c))?;
                            stdout().flush()?;
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

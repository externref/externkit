use crate::editor::editor::Editor;
use crossterm::{
    cursor, execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::ClearType,
};
use std::io::{self, stdout, Write};

pub struct Display;

impl Display {
    pub fn new() -> Self {
        Display
    }

    pub fn refresh_screen(&mut self, editor: &Editor) -> io::Result<()> {
        execute!(stdout(), cursor::Hide)?;
        execute!(stdout(), cursor::MoveTo(0, 0))?;

        let display_lines = if editor.terminal_height > 2 {
            editor.terminal_height - 2
        } else {
            0
        };

        for i in 0..display_lines {
            let line_index = editor.offset_y + i;
            if line_index < editor.content.len() {
                let line = &editor.content[line_index];
                let display_line = if line.len() > editor.terminal_width {
                    &line[..editor.terminal_width]
                } else {
                    line
                };
                execute!(
                    stdout(),
                    Print(format!(
                        "{:<width$}",
                        display_line,
                        width = editor.terminal_width
                    ))
                )?;
            } else {
                execute!(
                    stdout(),
                    Print(format!("{:<width$}", "~", width = editor.terminal_width))
                )?;
            }
            execute!(
                stdout(),
                crossterm::terminal::Clear(ClearType::UntilNewLine)
            )?;
            execute!(stdout(), Print("\r\n"))?;
        }

        self.draw_status_bar(editor)?;

        self.draw_help_bar(editor)?;

        let screen_y = editor.cursor_y - editor.offset_y;
        let display_x = if editor.cursor_x > 0 && editor.terminal_width > 0 && editor.cursor_x >= editor.terminal_width {
            editor.terminal_width.saturating_sub(1)
        } else {
            editor.cursor_x
        };

        execute!(stdout(), cursor::MoveTo(display_x as u16, screen_y as u16))?;
        execute!(stdout(), cursor::Show)?;
        stdout().flush()?;

        Ok(())
    }

    fn draw_status_bar(&self, editor: &Editor) -> io::Result<()> {
        execute!(stdout(), SetForegroundColor(Color::Black))?;
        execute!(stdout(), crossterm::style::SetBackgroundColor(Color::White))?;

        let status = format!(
            " {} | Line {}/{} | Col {} {}",
            editor.filename.as_deref().unwrap_or("New File"),
            editor.cursor_y + 1,
            editor.content.len(),
            editor.cursor_x + 1,
            if editor.modified { "[Modified]" } else { "" }
        );

        let truncated_status = if status.len() > editor.terminal_width {
            &status[..editor.terminal_width]
        } else {
            &status
        };

        execute!(
            stdout(),
            Print(format!(
                "{:<width$}",
                truncated_status,
                width = editor.terminal_width
            ))
        )?;
        execute!(stdout(), ResetColor)?;
        Ok(())
    }

    fn draw_help_bar(&self, editor: &Editor) -> io::Result<()> {
        execute!(stdout(), Print("\r\n"))?;
        execute!(stdout(), SetForegroundColor(Color::DarkGrey))?;

        let help = "^X Exit  ^S Save  ^O Open  Arrow keys to move";
        let truncated_help = if help.len() > editor.terminal_width {
            &help[..editor.terminal_width]
        } else {
            help
        };

        execute!(stdout(), Print(truncated_help))?;
        execute!(stdout(), ResetColor)?;
        Ok(())
    }
}

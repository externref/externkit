use crossterm::{
    cursor,
    event::{self, Event, KeyEventKind},
    execute,
    terminal::{self, ClearType},
};
use std::fs;
use std::io::{self, stdout};
use std::path::Path;

use crate::editor::display::Display;
use crate::editor::input::InputHandler;

pub struct Editor {
    pub content: Vec<String>,
    pub cursor_x: usize,
    pub cursor_y: usize,
    pub offset_y: usize,
    pub filename: Option<String>,
    pub modified: bool,
    pub terminal_height: usize,
    pub terminal_width: usize,
}

impl Editor {
    pub fn new() -> io::Result<Self> {
        let (width, height) = terminal::size()?;
        Ok(Editor {
            content: vec![String::new()],
            cursor_x: 0,
            cursor_y: 0,
            offset_y: 0,
            filename: None,
            modified: false,
            terminal_height: height as usize,
            terminal_width: width as usize,
        })
    }

    pub fn open_file(filename: &str) -> io::Result<Self> {
        let mut editor = Self::new()?;
        editor.filename = Some(filename.to_string());

        if Path::new(filename).exists() {
            let content = fs::read_to_string(filename)?;
            editor.content = if content.is_empty() {
                vec![String::new()]
            } else {
                content.lines().map(|s| s.to_string()).collect()
            };
        }

        Ok(editor)
    }

    pub fn run(&mut self) -> io::Result<()> {
        terminal::enable_raw_mode()?;
        execute!(stdout(), terminal::Clear(ClearType::All))?;

        let mut input_handler = InputHandler::new();

        loop {
            let display_result = {
                let mut display = Display::new();
                display.refresh_screen(self)
            };
            display_result?;

            if let Event::Key(key_event) = event::read()? {
                if key_event.kind == KeyEventKind::Press {
                    if input_handler.process_key(self, key_event)? {
                        break;
                    }
                }
            }
        }

        terminal::disable_raw_mode()?;
        execute!(stdout(), terminal::Clear(ClearType::All))?;
        execute!(stdout(), cursor::MoveTo(0, 0))?;
        Ok(())
    }

    pub fn clamp_cursor_x(&mut self) {
        let line_len = self.content[self.cursor_y].len();
        if self.cursor_x > line_len {
            self.cursor_x = line_len;
        }
    }

    pub fn insert_char(&mut self, c: char) {
        self.content[self.cursor_y].insert(self.cursor_x, c);
        self.cursor_x += 1;
        self.modified = true;
    }

    pub fn insert_newline(&mut self) {
        let current_line = &self.content[self.cursor_y];
        let new_line = current_line[self.cursor_x..].to_string();
        self.content[self.cursor_y].truncate(self.cursor_x);
        self.content.insert(self.cursor_y + 1, new_line);
        self.cursor_y += 1;
        self.cursor_x = 0;
        self.modified = true;
    }

    pub fn delete_char(&mut self) {
        if self.cursor_x > 0 {
            self.content[self.cursor_y].remove(self.cursor_x - 1);
            self.cursor_x -= 1;
            self.modified = true;
        } else if self.cursor_y > 0 {
            let current_line = self.content.remove(self.cursor_y);
            self.cursor_y -= 1;
            self.cursor_x = self.content[self.cursor_y].len();
            self.content[self.cursor_y].push_str(&current_line);
            self.modified = true;
        }
    }

    pub fn delete_char_forward(&mut self) {
        if self.cursor_x < self.content[self.cursor_y].len() {
            self.content[self.cursor_y].remove(self.cursor_x);
            self.modified = true;
        } else if self.cursor_y < self.content.len().saturating_sub(1) {
            let next_line = self.content.remove(self.cursor_y + 1);
            self.content[self.cursor_y].push_str(&next_line);
            self.modified = true;
        }
    }

    pub fn move_cursor_up(&mut self) {
        if self.cursor_y > 0 {
            self.cursor_y -= 1;
            if self.cursor_y < self.offset_y {
                self.offset_y = self.cursor_y;
            }
            self.clamp_cursor_x();
        }
    }

    pub fn move_cursor_down(&mut self) {
        if self.cursor_y < self.content.len().saturating_sub(1) {
            self.cursor_y += 1;
            // Prevent overflow by checking if terminal_height is large enough
            if self.terminal_height > 2 && self.cursor_y >= self.offset_y + self.terminal_height - 2 {
                self.offset_y = self.cursor_y.saturating_sub(self.terminal_height - 3);
            }
            self.clamp_cursor_x();
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor_x > 0 {
            self.cursor_x -= 1;
        } else if self.cursor_y > 0 {
            self.cursor_y -= 1;
            self.cursor_x = self.content[self.cursor_y].len();
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.cursor_x < self.content[self.cursor_y].len() {
            self.cursor_x += 1;
        } else if self.cursor_y < self.content.len() - 1 {
            self.cursor_y += 1;
            self.cursor_x = 0;
        }
    }
}

pub fn start_editor(filename: Option<&str>) -> io::Result<()> {
    let mut editor = if let Some(file) = filename {
        Editor::open_file(file)?
    } else {
        Editor::new()?
    };

    editor.run()
}

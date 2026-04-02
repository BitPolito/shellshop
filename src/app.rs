//! Ratatui shopping TUI — shared between SSH sessions and optional local mode.

use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crossterm::{
    event::{Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, ListState, Paragraph, Tabs},
    Terminal, TerminalOptions, Viewport,
};

const TAB_TITLES: &[&str] = &[" a account ", " c cart ", " s shop ", " h help "];

/// Single key event aligned with crossterm for shared handling logic.
#[derive(Debug, Clone, Copy)]
pub struct KeyEvent {
    pub code: KeyCode,
    pub modifiers: KeyModifiers,
}

/// Drain one logical key from `buf`, or `None` if more bytes are needed (incomplete escape).
fn pop_key(buf: &mut Vec<u8>) -> Option<KeyEvent> {
    if buf.is_empty() {
        return None;
    }
    // Ctrl+C
    if buf[0] == 0x03 {
        buf.remove(0);
        return Some(KeyEvent {
            code: KeyCode::Char('c'),
            modifiers: KeyModifiers::CONTROL,
        });
    }
    // Escape sequences (arrow keys, etc.)
    if buf[0] == 0x1b {
        if buf.len() >= 3 && buf[1] == b'[' {
            let code = match buf[2] {
                b'A' => KeyCode::Up,
                b'B' => KeyCode::Down,
                b'C' => KeyCode::Right,
                b'D' => KeyCode::Left,
                _ => {
                    // Unknown CSI: skip this byte triplet to avoid wedging the buffer
                    buf.drain(..3.min(buf.len()));
                    return pop_key(buf);
                }
            };
            buf.drain(..3);
            return Some(KeyEvent {
                code,
                modifiers: KeyModifiers::NONE,
            });
        }
        // SS3 arrows: ESC O A / ESC O B / ...
        if buf.len() >= 3 && buf[1] == b'O' {
            let code = match buf[2] {
                b'A' => KeyCode::Up,
                b'B' => KeyCode::Down,
                b'C' => KeyCode::Right,
                b'D' => KeyCode::Left,
                _ => {
                    buf.drain(..3.min(buf.len()));
                    return pop_key(buf);
                }
            };
            buf.drain(..3);
            return Some(KeyEvent {
                code,
                modifiers: KeyModifiers::NONE,
            });
        }
        // Incomplete escape — wait for more input
        return None;
    }
    let b = buf.remove(0);
    // Enter
    if b == b'\r' || b == b'\n' {
        return Some(KeyEvent {
            code: KeyCode::Enter,
            modifiers: KeyModifiers::NONE,
        });
    }
    let ch = char::from_u32(u32::from(b)).unwrap_or('\u{fffd}');
    Some(KeyEvent {
        code: KeyCode::Char(ch),
        modifiers: KeyModifiers::NONE,
    })
}

pub struct App {
    pub cols: u16,
    pub rows: u16,
    selected_tab: usize,
    account_list_state: ListState,
    is_light_mode: bool,
}

impl App {
    pub fn new(cols: u16, rows: u16) -> Self {
        let mut account_list_state = ListState::default();
        account_list_state.select(Some(0));
        Self {
            cols,
            rows,
            selected_tab: 0,
            account_list_state,
            is_light_mode: false,
        }
    }

    /// Run the TUI over stdin/stdout (local development).
    pub fn run_local() -> io::Result<()> {
        enable_raw_mode()?;
        let mut stdout = std::io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let mut app = App::new(terminal.size()?.width, terminal.size()?.height);
        let result = app.run_local_inner(&mut terminal);

        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        terminal.show_cursor()?;
        result
    }

    fn run_local_inner<W: Write>(&mut self, terminal: &mut Terminal<CrosstermBackend<W>>) -> io::Result<()> {
        // --- Loading Screen Loop ---
        let start_time = std::time::Instant::now();
        let loading_duration = Duration::from_secs(3);

        while start_time.elapsed() < loading_duration {
            while crossterm::event::poll(Duration::ZERO)? {
                if let Event::Key(key) = crossterm::event::read()? {
                    if (key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c'))
                        || key.code == KeyCode::Char('q')
                    {
                        return Ok(());
                    }
                }
            }

            self.draw_loading(terminal, start_time.elapsed().as_secs_f32())?;

            std::thread::sleep(Duration::from_millis(16));
        }

        // --- Main Loop ---
        let main_start_time = std::time::Instant::now();
        loop {
            if crossterm::event::poll(Duration::from_millis(16))? {
                if let Event::Key(key) = crossterm::event::read()? {
                    if (key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c'))
                        || key.code == KeyCode::Char('q')
                    {
                        break;
                    }
                    self.handle_key(key.code, key.modifiers);
                }
            }
            self.sync_viewport(terminal)?;
            self.draw_main(terminal, main_start_time.elapsed().as_secs_f64())?;
        }

        Ok(())
    }

    /// Run over an SSH-backed terminal; input arrives as raw bytes from the channel.
    pub fn run_over_channel<W: Write>(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<W>>,
        input: &std::sync::mpsc::Receiver<Vec<u8>>,
        dims: &Arc<Mutex<(u16, u16)>>,
    ) -> io::Result<()> {
        let mut pending = Vec::new();

        // --- Loading Screen Loop ---
        let start_time = std::time::Instant::now();
        let loading_duration = Duration::from_secs(3);

        while start_time.elapsed() < loading_duration {
            while let Ok(chunk) = input.try_recv() {
                pending.extend(chunk);
            }
            while let Some(ev) = pop_key(&mut pending) {
                if (ev.modifiers.contains(KeyModifiers::CONTROL) && ev.code == KeyCode::Char('c'))
                    || ev.code == KeyCode::Char('q')
                {
                    return Ok(());
                }
            }

            self.sync_viewport_from_dims(terminal, dims)?;
            self.draw_loading(terminal, start_time.elapsed().as_secs_f32())?;

            std::thread::sleep(Duration::from_millis(16));
        }

        // --- Main Loop ---
        let main_start_time = std::time::Instant::now();
        loop {
            match input.recv_timeout(Duration::from_millis(16)) {
                Ok(chunk) => pending.extend(chunk),
                Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {}
                Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => break,
            }

            while let Some(ev) = pop_key(&mut pending) {
                if (ev.modifiers.contains(KeyModifiers::CONTROL) && ev.code == KeyCode::Char('c'))
                    || ev.code == KeyCode::Char('q')
                {
                    return Ok(());
                }
                self.handle_key(ev.code, ev.modifiers);
            }

            self.sync_viewport_from_dims(terminal, dims)?;
            self.draw_main(terminal, main_start_time.elapsed().as_secs_f64())?;
        }

        Ok(())
    }

    fn sync_viewport<W: Write>(&mut self, terminal: &mut Terminal<CrosstermBackend<W>>) -> io::Result<()> {
        let s = terminal.size()?;
        if s.width != self.cols || s.height != self.rows {
            self.cols = s.width;
            self.rows = s.height;
            terminal.resize(Rect::new(0, 0, self.cols, self.rows))?;
        }
        Ok(())
    }

    fn sync_viewport_from_dims<W: Write>(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<W>>,
        dims: &Arc<Mutex<(u16, u16)>>,
    ) -> io::Result<()> {
        let (c, r) = *dims.lock().unwrap();
        if c != self.cols || r != self.rows {
            self.cols = c;
            self.rows = r;
            terminal.resize(Rect::new(0, 0, self.cols, self.rows))?;
        }
        Ok(())
    }

    fn handle_key(&mut self, code: KeyCode, modifiers: KeyModifiers) {
        match code {
            KeyCode::Right => {
                self.selected_tab = (self.selected_tab + 1) % 4;
            }
            KeyCode::Left => {
                self.selected_tab = (self.selected_tab + 3) % 4;
            }
            KeyCode::Char('d') => {
                if self.selected_tab == 0 {
                    let i = match self.account_list_state.selected() {
                        Some(i) if i >= 5 => 0,
                        Some(i) => i + 1,
                        None => 0,
                    };
                    self.account_list_state.select(Some(i));
                }
            }
            KeyCode::Char('a') => {
                if self.selected_tab == 0 {
                    let i = match self.account_list_state.selected() {
                        Some(0) => 5,
                        Some(i) => i - 1,
                        None => 0,
                    };
                    self.account_list_state.select(Some(i));
                } else {
                    self.selected_tab = 0;
                }
            }
            KeyCode::Char('c') if !modifiers.contains(KeyModifiers::CONTROL) => self.selected_tab = 1,
            KeyCode::Char('s') => self.selected_tab = 2,
            KeyCode::Char('h') => self.selected_tab = 3,
            KeyCode::Enter => {
                if self.selected_tab == 0 {
                    if let Some(5) = self.account_list_state.selected() {
                        self.is_light_mode = !self.is_light_mode;
                    }
                }
            }
            _ => {}
        }
    }

    fn draw_loading<W: Write>(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<W>>,
        elapsed_secs: f32,
    ) -> io::Result<()> {
        let progress = (elapsed_secs / 3.0).min(1.0);
        terminal.draw(|f| {
            let size = f.area();

            let text = Paragraph::new(
                "
  ▄█████ ▄▄ ▄▄ ▄▄▄▄▄ ▄▄    ▄▄    ▄█████ ▄▄ ▄▄  ▄▄▄  ▄▄▄▄
  ▀▀▀▄▄▄ ██▄██ ██▄▄  ██    ██    ▀▀▀▄▄▄ ██▄██ ██▀██ ██▄█▀
█████▀ ██ ██ ██▄▄▄ ██▄▄▄ ██▄▄▄ █████▀ ██ ██ ▀███▀ ██
            ",
            )
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD));

            let area = centered_rect(40, 25, size);

            let layout = ratatui::layout::Layout::default()
                .direction(ratatui::layout::Direction::Vertical)
                .constraints([
                    ratatui::layout::Constraint::Length(5),
                    ratatui::layout::Constraint::Length(2),
                    ratatui::layout::Constraint::Length(1),
                    ratatui::layout::Constraint::Min(0),
                ])
                .split(area);

            f.render_widget(text, layout[0]);

            let gauge = Gauge::default()
                .gauge_style(Style::default().fg(Color::Blue).bg(Color::DarkGray))
                .percent((progress * 100.0) as u16);

            f.render_widget(gauge, layout[2]);
        })?;
        Ok(())
    }

    fn draw_main<W: Write>(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<W>>,
        main_elapsed: f64,
    ) -> io::Result<()> {
        let selected_tab = self.selected_tab;
        let is_light_mode = self.is_light_mode;
        let account_list_state = &self.account_list_state;

        terminal.draw(|f| {
            let size = f.area();

            let bg_color = if is_light_mode {
                Color::White
            } else {
                Color::Reset
            };
            let fg_color = if is_light_mode {
                Color::Black
            } else {
                Color::White
            };

            f.render_widget(ratatui::widgets::Clear, size);
            f.render_widget(
                Block::default().style(Style::default().bg(bg_color).fg(fg_color)),
                size,
            );

            let vertical_chunks = ratatui::layout::Layout::default()
                .direction(ratatui::layout::Direction::Vertical)
                .constraints([
                    ratatui::layout::Constraint::Percentage(10),
                    ratatui::layout::Constraint::Percentage(80),
                    ratatui::layout::Constraint::Percentage(10),
                ])
                .split(size);

            let area = ratatui::layout::Layout::default()
                .direction(ratatui::layout::Direction::Horizontal)
                .constraints([
                    ratatui::layout::Constraint::Percentage(5),
                    ratatui::layout::Constraint::Percentage(90),
                    ratatui::layout::Constraint::Percentage(5),
                ])
                .split(vertical_chunks[1])[1];

            let binary_text = "01100011 01111001 01110000 01101000 01100101 01110010 01110000 01110101 01101110 01101011 01110011  01110111 01110010 01101001 01110100 01100101  01100011 01101111 01100100 01100101";
            let block = Block::default()
                .title_bottom(binary_text)
                .title_alignment(Alignment::Right)
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Thick)
                .style(Style::default().fg(Color::Blue));

            let inner_area = block.inner(area);

            let t_head = (main_elapsed * 80.0) as usize;

            let w = area.width as usize;
            let h = area.height as usize;
            let perimeter = (2 * w + 2 * h).saturating_sub(4);
            let text_len = binary_text.len();

            if w > 1 && h > 1 && t_head < perimeter + text_len {
                let buf = f.buffer_mut();
                let thick_sym = ratatui::symbols::line::THICK;
                let text_bytes = binary_text.as_bytes();

                for d in 0..perimeter {
                    if d > t_head {
                        continue;
                    }

                    let (cx, cy, ch) = if d < w {
                        let i = d;
                        let cx = area.x + (w - 1 - i) as u16;
                        let cy = area.y + (h - 1) as u16;
                        let ch = if i == 0 {
                            thick_sym.bottom_right
                        } else if i == w - 1 {
                            thick_sym.bottom_left
                        } else {
                            thick_sym.horizontal
                        };
                        (cx, cy, ch)
                    } else if d < w + h - 1 {
                        let i = d - w + 1;
                        let cx = area.x;
                        let cy = area.y + (h - 1 - i) as u16;
                        let ch = if i == h - 1 {
                            thick_sym.top_left
                        } else {
                            thick_sym.vertical
                        };
                        (cx, cy, ch)
                    } else if d < 2 * w + h - 2 {
                        let i = d - (w + h - 1) + 1;
                        let cx = area.x + i as u16;
                        let cy = area.y;
                        let ch = if i == w - 1 {
                            thick_sym.top_right
                        } else {
                            thick_sym.horizontal
                        };
                        (cx, cy, ch)
                    } else {
                        let i = d - (2 * w + h - 2) + 1;
                        let cx = area.x + (w - 1) as u16;
                        let cy = area.y + i as u16;
                        (cx, cy, thick_sym.vertical)
                    };

                    if cx < size.width && cy < size.height {
                        if let Some(cell) = buf.cell_mut((cx, cy)) {
                            if t_head >= d && t_head - d < text_len {
                                cell.set_char(text_bytes[t_head - d] as char)
                                    .set_fg(Color::Blue);
                            } else {
                                cell.set_symbol(ch).set_fg(Color::Blue);
                            }
                        }
                    }
                }
            } else {
                f.render_widget(block, area);
            }

            let inner_chunks = ratatui::layout::Layout::default()
                .direction(ratatui::layout::Direction::Vertical)
                .constraints([
                    ratatui::layout::Constraint::Length(2),
                    ratatui::layout::Constraint::Min(0),
                ])
                .split(inner_area);

            let titles: Vec<Line> = TAB_TITLES
                .iter()
                .map(|t| Line::from(Span::raw(*t)))
                .collect();

            let tabs_width: u16 = TAB_TITLES
                .iter()
                .map(|t| t.len() as u16)
                .sum::<u16>()
                + (TAB_TITLES.len() as u16) * 2;
            let center_offset = inner_chunks[0].width.saturating_sub(tabs_width) / 2;

            let tabs_layout = ratatui::layout::Layout::default()
                .direction(ratatui::layout::Direction::Horizontal)
                .constraints([
                    ratatui::layout::Constraint::Length(center_offset),
                    ratatui::layout::Constraint::Min(tabs_width),
                ])
                .split(inner_chunks[0]);

            let tabs = Tabs::new(titles)
                .block(Block::default().padding(ratatui::widgets::Padding::horizontal(2)))
                .select(selected_tab)
                .highlight_style(
                    Style::default()
                        .fg(fg_color)
                        .bg(Color::Blue)
                        .add_modifier(Modifier::BOLD),
                );

            f.render_widget(tabs, tabs_layout[1]);

            if selected_tab == 0 {
                let options = [
                    "profile",
                    "payments",
                    "favorites",
                    "Option 4",
                    "Option 5",
                    "dark/light",
                ];
                let sub_titles: Vec<Line> = options
                    .iter()
                    .map(|t| Line::from(Span::raw(*t)))
                    .collect();

                let sub_tabs_width: u16 = options.iter().map(|t| t.len() as u16).sum::<u16>()
                    + (options.len() as u16) * 2;
                let sub_center_offset = inner_chunks[1].width.saturating_sub(sub_tabs_width) / 2;

                let sub_tabs = Tabs::new(sub_titles)
                    .select(account_list_state.selected().unwrap_or(0))
                    .highlight_style(
                        Style::default()
                            .bg(Color::Blue)
                            .fg(fg_color)
                            .add_modifier(Modifier::BOLD),
                    )
                    .divider("|");

                let sub_tabs_v_layout = ratatui::layout::Layout::default()
                    .direction(ratatui::layout::Direction::Vertical)
                    .constraints([
                        ratatui::layout::Constraint::Length(0),
                        ratatui::layout::Constraint::Length(1),
                        ratatui::layout::Constraint::Min(0),
                    ])
                    .split(inner_chunks[1]);

                let sub_tabs_h_layout = ratatui::layout::Layout::default()
                    .direction(ratatui::layout::Direction::Horizontal)
                    .constraints([
                        ratatui::layout::Constraint::Length(sub_center_offset),
                        ratatui::layout::Constraint::Min(sub_tabs_width),
                    ])
                    .split(sub_tabs_v_layout[1]);

                f.render_widget(sub_tabs, sub_tabs_h_layout[1]);
            } else {
                let text = match selected_tab {
                    3 => Paragraph::new(
                        "
  ▄█████ ▄▄ ▄▄ ▄▄▄▄▄ ▄▄    ▄▄    ▄█████ ▄▄ ▄▄  ▄▄▄  ▄▄▄▄
  ▀▀▀▄▄▄ ██▄██ ██▄▄  ██    ██    ▀▀▀▄▄▄ ██▄██ ██▀██ ██▄█▀
█████▀ ██ ██ ██▄▄▄ ██▄▄▄ ██▄▄▄ █████▀ ██ ██ ▀███▀ ██

←/→ or a/c/s/h to navigate tabs
a/d to select items
enter to select
q to quit
                    ",
                    )
                    .alignment(Alignment::Center)
                    .style(Style::default().fg(fg_color)),
                    2 => Paragraph::new("\nShop items")
                        .alignment(Alignment::Center)
                        .style(Style::default().fg(fg_color).add_modifier(Modifier::BOLD)),
                    1 => Paragraph::new("\nCart is empty")
                        .alignment(Alignment::Center)
                        .style(Style::default().fg(fg_color).add_modifier(Modifier::BOLD)),
                    _ => Paragraph::new(""),
                };

                f.render_widget(text, inner_chunks[1]);
            }
        })?;
        Ok(())
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = ratatui::layout::Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            ratatui::layout::Constraint::Percentage((100 - percent_y) / 2),
            ratatui::layout::Constraint::Percentage(percent_y),
            ratatui::layout::Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    ratatui::layout::Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([
            ratatui::layout::Constraint::Percentage((100 - percent_x) / 2),
            ratatui::layout::Constraint::Percentage(percent_x),
            ratatui::layout::Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

/// Build a ratatui terminal for an SSH session with the given viewport size.
pub fn terminal_for_ssh<W: Write>(
    writer: W,
    cols: u16,
    rows: u16,
) -> io::Result<Terminal<CrosstermBackend<W>>> {
    let backend = CrosstermBackend::new(writer);
    let options = TerminalOptions {
        viewport: Viewport::Fixed(Rect::new(0, 0, cols, rows)),
    };
    Terminal::with_options(backend, options)
}

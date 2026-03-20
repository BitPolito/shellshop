use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Tabs, List, ListItem, ListState, Gauge},
    Terminal,
};
use std::io::{self, stdout};

fn main() -> Result<(), io::Error> {
    // --- Setup Terminal ---
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut selected_tab = 0; // 0 to 3
    let tab_titles = vec![" a account ", " c cart ", " s shop ", " h help "];
    let mut account_list_state = ListState::default();
    account_list_state.select(Some(0));
    let mut is_light_mode = false;

    // --- Loading Screen Loop ---
    let start_time = std::time::Instant::now();
    let loading_duration = std::time::Duration::from_secs(3);

    while start_time.elapsed() < loading_duration {
        if event::poll(std::time::Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if (key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c'))
                    || key.code == KeyCode::Char('q')
                {
                    disable_raw_mode()?;
                    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
                    terminal.show_cursor()?;
                    return Ok(());
                }
            }
        }

        terminal.draw(|f| {
            let size = f.area();
            
            let elapsed = start_time.elapsed().as_secs_f32();
            let progress = (elapsed / 3.0).min(1.0);
            
            let text = Paragraph::new("
  ▄█████ ▄▄ ▄▄ ▄▄▄▄▄ ▄▄    ▄▄    ▄█████ ▄▄ ▄▄  ▄▄▄  ▄▄▄▄
  ▀▀▀▄▄▄ ██▄██ ██▄▄  ██    ██    ▀▀▀▄▄▄ ██▄██ ██▀██ ██▄█▀
█████▀ ██ ██ ██▄▄▄ ██▄▄▄ ██▄▄▄ █████▀ ██ ██ ▀███▀ ██
            ")
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD));

            let area = centered_rect(40, 25, size);
            
            let layout = ratatui::layout::Layout::default()
                .direction(ratatui::layout::Direction::Vertical)
                .constraints([
                    ratatui::layout::Constraint::Length(5), // ASCII ART Height
                    ratatui::layout::Constraint::Length(2), // Spacer
                    ratatui::layout::Constraint::Length(1), // Progress Bar
                    ratatui::layout::Constraint::Min(0),
                ])
                .split(area);

            f.render_widget(text, layout[0]);
            
            let gauge = Gauge::default()
                .gauge_style(Style::default().fg(Color::Blue).bg(Color::DarkGray))
                .percent((progress * 100.0) as u16);
            
            f.render_widget(gauge, layout[2]);
        })?;
    }

    // --- Main Loop ---
    let main_start_time = std::time::Instant::now();
    loop {
        // Handle input events
        if event::poll(std::time::Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                // Quit on Ctrl+C or 'q'
                if (key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c'))
                    || key.code == KeyCode::Char('q')
                {
                    break;
                }

                // Navigate tabs with Left/Right arrows and letter shortcuts
                match key.code {
                    KeyCode::Right => {
                        selected_tab = (selected_tab + 1) % 4;
                    }
                    KeyCode::Left => {
                        selected_tab = (selected_tab + 3) % 4;
                    }
                    KeyCode::Char('d') => {
                        if selected_tab == 0 {
                            let i = match account_list_state.selected() {
                                Some(i) => if i >= 5 { 0 } else { i + 1 },
                                None => 0,
                            };
                            account_list_state.select(Some(i));
                        }
                    }
                    KeyCode::Char('a') => {
                        if selected_tab == 0 {
                            let i = match account_list_state.selected() {
                                Some(i) => if i == 0 { 5 } else { i - 1 },
                                None => 0,
                            };
                            account_list_state.select(Some(i));
                        } else {
                            selected_tab = 0;
                        }
                    }
                    KeyCode::Char('c') => selected_tab = 1,
                    KeyCode::Char('s') => selected_tab = 2,
                    KeyCode::Char('h') => selected_tab = 3,
                    KeyCode::Enter => {
                        if selected_tab == 0 {
                            if let Some(5) = account_list_state.selected() {
                                is_light_mode = !is_light_mode;
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        terminal.draw(|f| {
            let size = f.area();
            
            let bg_color = if is_light_mode { Color::White } else { Color::Reset };
            let fg_color = if is_light_mode { Color::Black } else { Color::White };
            
            f.render_widget(ratatui::widgets::Clear, size);
            f.render_widget(Block::default().style(Style::default().bg(bg_color).fg(fg_color)), size);

            // 1. Define the vertical layout for the entire app area
            let vertical_chunks = ratatui::layout::Layout::default()
                .direction(ratatui::layout::Direction::Vertical)
                .constraints([
                    ratatui::layout::Constraint::Percentage(10), // Top margin
                    ratatui::layout::Constraint::Percentage(80), // Main Box Height
                    ratatui::layout::Constraint::Percentage(10), // Bottom margin
                ])
                .split(size);

            // 2. Define the horizontal layout: 5% margin sides, 90% center
            let area = ratatui::layout::Layout::default()
                .direction(ratatui::layout::Direction::Horizontal)
                .constraints([
                    ratatui::layout::Constraint::Percentage(5),  // Left margin
                    ratatui::layout::Constraint::Percentage(90), // Main Box Width
                    ratatui::layout::Constraint::Percentage(5),  // Right margin
                ])
                .split(vertical_chunks[1])[1]; // Split the middle vertical chunk

            // 3. Create the Large Block (the border blue box)
            let binary_text = "01100011 01111001 01110000 01101000 01100101 01110010 01110000 01110101 01101110 01101011 01110011  01110111 01110010 01101001 01110100 01100101  01100011 01101111 01100100 01100101";
            let block = Block::default()
                .title_bottom(binary_text)
                .title_alignment(Alignment::Right)
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Thick) // Makes the box look beefier
                .style(Style::default().fg(Color::Blue));

            // Obtain inner area for contents (tabs and text)
            let inner_area = block.inner(area);

            let elapsed = main_start_time.elapsed().as_secs_f64();
            let chars_per_sec = 80.0;
            let t_head = (elapsed * chars_per_sec) as usize;
            
            let w = area.width as usize;
            let h = area.height as usize;
            let perimeter = (2 * w + 2 * h).saturating_sub(4);
            let text_len = binary_text.len();
            
            if w > 1 && h > 1 && t_head < perimeter + text_len {
                let buf = f.buffer_mut();
                let thick_sym = ratatui::symbols::line::THICK;
                let text_bytes = binary_text.as_bytes();
                
                for d in 0..perimeter {
                    if d > t_head { continue; }
                    
                    let (cx, cy, ch) = if d < w {
                        let i = d;
                        let cx = area.x + (w - 1 - i) as u16;
                        let cy = area.y + (h - 1) as u16;
                        let ch = if i == 0 { thick_sym.bottom_right } else if i == w - 1 { thick_sym.bottom_left } else { thick_sym.horizontal };
                        (cx, cy, ch)
                    } else if d < w + h - 1 {
                        let i = d - w + 1;
                        let cx = area.x;
                        let cy = area.y + (h - 1 - i) as u16;
                        let ch = if i == h - 1 { thick_sym.top_left } else { thick_sym.vertical };
                        (cx, cy, ch)
                    } else if d < 2 * w + h - 2 {
                        let i = d - (w + h - 1) + 1;
                        let cx = area.x + i as u16;
                        let cy = area.y;
                        let ch = if i == w - 1 { thick_sym.top_right } else { thick_sym.horizontal };
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
                                cell.set_char(text_bytes[t_head - d] as char).set_fg(Color::Blue);
                            } else {
                                cell.set_symbol(ch).set_fg(Color::Blue);
                            }
                        }
                    }
                }
            } else {
                // Render the outer blue box border normally
                f.render_widget(block, area);
            }

            // Main inner chunks: 2 rows for Tabs, the rest for content
            let inner_chunks = ratatui::layout::Layout::default()
                .direction(ratatui::layout::Direction::Vertical)
                .constraints([
                    ratatui::layout::Constraint::Length(2), // Tab bar height
                    ratatui::layout::Constraint::Min(0),    // Rest of app
                ])
                .split(inner_area);

            // --- Create Tabs ---
            let titles: Vec<Line> = tab_titles
                .iter()
                .map(|t| Line::from(Span::raw(*t)))
                .collect();

            // Calculate width needed for tabs to pseudo-center them
            let tabs_width: u16 = tab_titles.iter().map(|t| t.len() as u16).sum::<u16>() + (tab_titles.len() as u16) * 2;
            let center_offset = inner_chunks[0].width.saturating_sub(tabs_width) / 2;

            let tabs_layout = ratatui::layout::Layout::default()
                .direction(ratatui::layout::Direction::Horizontal)
                .constraints([
                    ratatui::layout::Constraint::Length(center_offset),
                    ratatui::layout::Constraint::Min(tabs_width),
                ])
                .split(inner_chunks[0]);

            let tabs = Tabs::new(titles)
                // Use a block to add margin if needed, but no borders so it fits nicely
                .block(Block::default().padding(ratatui::widgets::Padding::horizontal(2)))
                .select(selected_tab)
                .highlight_style(
                    Style::default()
                        .fg(fg_color)
                        .bg(Color::Blue) // Highlight/hover color
                        .add_modifier(Modifier::BOLD),
                );

            f.render_widget(tabs, tabs_layout[1]);

            if selected_tab == 0 {
                let options = vec![
                    "profile", "payments", "favorites", 
                    "Option 4", "Option 5", "dark/light"
                ];
                let sub_titles: Vec<Line> = options.iter().map(|t| Line::from(Span::raw(*t))).collect();
                
                let sub_tabs_width: u16 = options.iter().map(|t| t.len() as u16).sum::<u16>() + (options.len() as u16) * 2;
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
                        ratatui::layout::Constraint::Length(0), // slight gap to rest directly below first tab list
                        ratatui::layout::Constraint::Length(1), // exactly 1 height for horizontal list
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
                    3 => Paragraph::new("
  ▄█████ ▄▄ ▄▄ ▄▄▄▄▄ ▄▄    ▄▄    ▄█████ ▄▄ ▄▄  ▄▄▄  ▄▄▄▄
  ▀▀▀▄▄▄ ██▄██ ██▄▄  ██    ██    ▀▀▀▄▄▄ ██▄██ ██▀██ ██▄█▀
█████▀ ██ ██ ██▄▄▄ ██▄▄▄ ██▄▄▄ █████▀ ██ ██ ▀███▀ ██

←/→ or a/c/s/h to navigate tabs
a/d to select items
enter to select
q to quit
                    ")
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
    }

    // --- Restore Terminal ---
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

/// Helper function to create a centered rectangle
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

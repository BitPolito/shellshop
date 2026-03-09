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
    widgets::{Block, Borders, Paragraph, Tabs},
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
    let tab_titles = vec![" Option 1 ", " Option 2 ", " Option 3 ", " Option 4 "];

    // --- Main Loop ---
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

                // Navigate tabs with Left/Right arrows
                match key.code {
                    KeyCode::Right => {
                        selected_tab = (selected_tab + 1) % 4;
                    }
                    KeyCode::Left => {
                        selected_tab = (selected_tab + 3) % 4;
                    }
                    _ => {}
                }
            }
        }

        terminal.draw(|f| {
            let size = f.area();

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
            let block = Block::default()
                .title_bottom("01100011 01111001 01110000 01101000 01100101 01110010 01110000 01110101 01101110 01101011 01110011  01110111 01110010 01101001 01110100 01100101  01100011 01101111 01100100 01100101")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Thick) // Makes the box look beefier
                .style(Style::default().fg(Color::Blue));

            // Obtain inner area for contents (tabs and text)
            let inner_area = block.inner(area);

            // Render the outer blue box border
            f.render_widget(block, area);

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
                        .fg(Color::Black)
                        .bg(Color::Cyan) // Highlight/hover color
                        .add_modifier(Modifier::BOLD),
                );

            f.render_widget(tabs, tabs_layout[1]);

            // 4. Render the Paragraph
            let text = Paragraph::new("
  ▄█████ ▄▄ ▄▄ ▄▄▄▄▄ ▄▄    ▄▄    ▄█████ ▄▄ ▄▄  ▄▄▄  ▄▄▄▄
  ▀▀▀▄▄▄ ██▄██ ██▄▄  ██    ██    ▀▀▀▄▄▄ ██▄██ ██▀██ ██▄█▀
█████▀ ██ ██ ██▄▄▄ ██▄▄▄ ██▄▄▄ █████▀ ██ ██ ▀███▀ ██
            ")
                .alignment(Alignment::Center)
                .style(Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD)
                );

            f.render_widget(text, inner_chunks[1]);
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

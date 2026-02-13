use ratatui::{DefaultTerminal, Frame};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let app_result = app(&mut terminal);
    ratatui::restore();
    Ok(app_result?)
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(render)?;
        if crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    frame.render_widget("This is Polify!", frame.area());
}

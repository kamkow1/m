use std::{io, error::Error};
use tui::{
    backend::{CrosstermBackend, Backend},
    widgets::{Block, Borders},
    Terminal
};
use crossterm::{
    event::{
        self,
        DisableMouseCapture,
        EnableMouseCapture,
        Event, 
        KeyCode
    },
    terminal::{
        disable_raw_mode,
        enable_raw_mode,
        EnterAlternateScreen,
        LeaveAlternateScreen
    },
    execute,
};

fn render_frame<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    terminal.draw(|frame| {
        let size = frame.size();
        let block = Block::default()
            .title("eeoeoe")
            .borders(Borders::ALL);
        frame.render_widget(block, size);
    })?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;


    'program_loop: loop {
        render_frame(&mut terminal).expect("Failed to render frame");

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break 'program_loop,
                _ => {},
            }
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    terminal.show_cursor()?;

    Ok(())
}

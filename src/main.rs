#![feature(absolute_path)]

use std::{path::Path, fs, env, io, error::Error};
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

struct App<'a> {
    sound_filename: &'a str,
}

impl App<'_> {
    fn render_frame<B: Backend>(&self, terminal: &mut Terminal<B>) -> io::Result<()> {
        terminal.draw(|frame| {
            let size = frame.size();
            let block = Block::default()
                .title(self.sound_filename)
                .borders(Borders::ALL);
            frame.render_widget(block, size);
        })?;

        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    let input = env::args().nth(1).expect("Sound filename not provided");
    //let p = Path::new(&);
    let canonical = fs::canonicalize(&input)?;
    let sound_filename = match canonical.to_str() {
        Some(x) => x,
        None => "Failed to canonicalize sound filename",
    };
    let app = App{ sound_filename };

    'program_loop: loop {
        app.render_frame(&mut terminal)?;

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

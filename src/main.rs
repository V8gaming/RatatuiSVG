use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, time::Duration, vec, fs::File, io::Write};
use ratatui::{
    backend::CrosstermBackend,
    text::Spans,
    Terminal,
};
use std::sync::atomic::AtomicI32;
use crate::draw::draw;

/// The index of the currently selected tab.
static INDEX: AtomicI32 = AtomicI32::new(0);
mod svg;
mod draw;
/// The SVGs to render.
const STRINGS: [&str; 7] = [
    r#"<path d="M 50.000 75.000 V 25.000 M 25.000 50.000 H 75.000" style="stroke: rgb(255, 0, 0); stroke-width: 1; fill: none;"/>"#,
    r#"<path d="M 25.000 25.000 L 75.000 25.000 L 75.000 75.000 L 25.000 75.000 Z" style="stroke: rgb(255, 0, 0); stroke-width: 1; fill: none;"/>"#,
    r#"<path d="M 25.000 25.000 C 25.000 30.000 75.000 27.000 45.000 25.000" style="stroke: rgb(255, 0, 0); stroke-width: 1; fill: none;"/>"#,
    r#"<path d="M 25.000 25.000 C 25.000 30.000 75.000 27.000 45.000 50.000" style="stroke: rgb(255, 0, 0); stroke-width: 1; fill: none;"/>"#,
    r#"<path d="M 25.000 25.000 A 10.000 10.000 90.000 0 0 50.000 50.000" style="stroke: rgb(255, 0, 0); stroke-width: 1; fill: none;"/>"#,
    r#"<path d="M 25.000 25.000 A 10.000 10.000 90.000 1 0 50.000 50.000" style="stroke: rgb(255, 0, 0); stroke-width: 1; fill: none;"/>"#,
    r#"<path d="M 25.000 25.000 A 10.000 10.000 90.000 0 1 50.000 50.000" style="stroke: rgb(255, 0, 0); stroke-width: 1; fill: none;"/>"#,
];
/// The main function.
fn main() -> Result<(), io::Error>{
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut svgs: Vec<Vec<&str>> = Vec::new();
    for i in STRINGS {
        svgs.push(vec![i]);
    }

    let mut tabs = Vec::new();

    for i in 0..svgs.clone().len() {
        tabs.push(Spans::from(format!("Tab {i}")));
    }
    
    loop {
        read_input(&mut terminal, tabs.clone().len() as i32)?;
        draw(&mut terminal, tabs.clone(), svgs.clone())?;
        //println!("{}", tabs.clone().len());
    }

}

/// Using the api built into crossterm, we can read keyinputs.
pub fn read_input(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    tabs: i32,
) -> Result<(), io::Error> {
    if event::poll(Duration::from_millis(100))? {
        match event::read()? {
            Event::Key(event) => {
                if event.code == KeyCode::Char('Q') && event.modifiers == event::KeyModifiers::SHIFT
                {
                    // clear the screen
                    execute!(
                        terminal.backend_mut(),
                        LeaveAlternateScreen,
                        DisableMouseCapture
                    )?;
                    disable_raw_mode()?;
                    terminal.show_cursor()?;
                    terminal.clear()?;

                    Err(io::Error::new(io::ErrorKind::Other, "Quit"))                
                } else if event.code == KeyCode::Char('c')
                    && event.modifiers == event::KeyModifiers::CONTROL
                {
                    // clear the screen
                    execute!(
                        terminal.backend_mut(),
                        LeaveAlternateScreen,
                        DisableMouseCapture
                    )?;
                    disable_raw_mode()?;
                    terminal.show_cursor()?;
                    terminal.clear()?;
                    return Err(io::Error::new(io::ErrorKind::Other, "Quit"));
                } else {
                    match event.code {
                        KeyCode::Right => {
                            if INDEX.load(std::sync::atomic::Ordering::Relaxed) + 1  > tabs - 1 {
                                INDEX.store(0, std::sync::atomic::Ordering::Relaxed) ;
                            } else {
                                INDEX.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                            }
                            

                            return Ok(());
                        }
                        KeyCode::Left => {
                            if INDEX.load(std::sync::atomic::Ordering::Relaxed) - 1 < 0 {
                                INDEX.store(tabs - 1, std::sync::atomic::Ordering::Relaxed);
                            } else {
                                INDEX.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
                            }
                            return Ok(());
                        }
                        KeyCode::Enter => {
                            let f = File::create("current.svg").unwrap();
                            let mut f = std::io::BufWriter::new(f);
                            writeln!(f, "<?xml version=\"1.0\" encoding=\"utf-8\"?>").unwrap();
                            writeln!(f, "<svg viewBox=\"0 0 100 100\" xmlns=\"http://www.w3.org/2000/svg\">").unwrap();
                            writeln!(f, "{}", STRINGS[INDEX.load(std::sync::atomic::Ordering::Relaxed) as usize]).unwrap();
                            writeln!(f, "</svg>").unwrap();
                            return Ok(());
                            
                        }
                        _ => return Ok(()),
                    }
                }
            }
            _ => Ok(()),
        }
    } else {
        Ok(())
    }
}

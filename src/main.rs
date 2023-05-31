use canvas::Svg;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use itertools::Itertools;
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{fs::File, io, io::Write, sync::atomic::AtomicI32, time::Duration, vec, collections::HashMap};
/// The index of the currently selected tab.
static INDEX: AtomicI32 = AtomicI32::new(0);
mod canvas;
mod draw;
mod svg;

/// The main function.
fn main() -> Result<(), io::Error> {
    let mut svgs = Svg::new();
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    svgs.add("MHV".to_string(),vec![r#"<path d="M 50.000 75.000 V 25.000 M 25.000 50.000 H 75.000" style="stroke: rgb(255, 0, 0); stroke-width: 1; fill: none;"/>"#.to_string(),]);
    svgs.add("L".to_string(), vec![r#"<path d="M 25.000 25.000 L 75.000 25.000 L 75.000 75.000 L 25.000 75.000 Z" style="stroke: rgb(255, 0, 0); stroke-width: 1; fill: none;"/>"#.to_string(),]);
    svgs.add("C1".to_string(), vec![r#"<path d="M 25.000 25.000 C 25.000 30.000 75.000 27.000 45.000 25.000" style="stroke: rgb(255, 0, 0); stroke-width: 1; fill: none;"/>"#.to_string(),]);
    svgs.add("C2".to_string(), vec![r#"<path d="M 25.000 25.000 C 25.000 30.000 75.000 27.000 45.000 50.000" style="stroke: rgb(255, 0, 0); stroke-width: 1; fill: none;"/>"#.to_string(),]);
    svgs.add("A".to_string(), vec![r#"<path d="M 25.000 25.000 A 10.000 10.000 90.000 0 0 50.000 50.000" style="stroke: rgb(255, 0, 0); stroke-width: 1; fill: none;"/>"#.to_string(),]);
    svgs.add("AF1".to_string(), vec![r#"<path d="M 25.000 25.000 A 10.000 10.000 90.000 1 0 50.000 50.000" style="stroke: rgb(255, 0, 0); stroke-width: 1; fill: none;"/>"#.to_string(),]);
    svgs.add("AF2".to_string(), vec![r#"<path d="M 25.000 25.000 A 10.000 10.000 90.000 0 1 50.000 50.000" style="stroke: rgb(255, 0, 0); stroke-width: 1; fill: none;"/>"#.to_string(),]);
    svgs.add("Rect".to_string(), vec![r#"<rect x="25.000" y="25.000" width="50.000" height="50.000" style="stroke: rgb(255, 0, 0); stroke-width: 1; fill: none;"/>"#.to_string(),]);
    svgs.add(
        "Line".to_string(),
    vec![
            r#"<line x1="25.000" y1="25.000" x2="75.000" y2="75.000" style="stroke: rgb(255, 0, 0); stroke-width: 1; fill: none;"/>"#.to_string(),
            r#"<line x1="75.000" y1="25.000" x2="25.000" y2="75.000" style="stroke: rgb(0, 0, 255); stroke-width: 1; fill: none;"/>"#.to_string()
        ]
    );

    svgs.import("test.svg".to_string(), String::new());
    svgs.import("current.svg".to_string(), String::new());
    svgs.initialize();
    loop {
        read_input(&mut terminal, svgs.as_hashmap().to_owned()).ok();
        svgs.run(&mut terminal);
    }
}

/// Using the api built into crossterm, we can read keyinputs.
pub fn read_input(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    svgs: HashMap<String, Vec<String>>,
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
                            if INDEX.load(std::sync::atomic::Ordering::Relaxed) + 1
                                > svgs.len() as i32 - 1
                            {
                                INDEX.store(0, std::sync::atomic::Ordering::Relaxed);
                            } else {
                                INDEX.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                            }

                            return Ok(());
                        }
                        KeyCode::Left => {
                            if INDEX.load(std::sync::atomic::Ordering::Relaxed) - 1 < 0 {
                                INDEX.store(
                                    svgs.len() as i32 - 1,
                                    std::sync::atomic::Ordering::Relaxed,
                                );
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
                            let svg_string = svgs
                                .get(
                                    &svgs.keys().sorted().cloned().collect::<Vec<String>>()
                                        [INDEX.load(std::sync::atomic::Ordering::Relaxed) as usize],
                                )
                                .unwrap().to_owned();
                            for i in &svg_string {
                                writeln!(f, "{i}").unwrap();
                            }

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

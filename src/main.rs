use crate::draw::draw;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, text::Spans, Terminal};
use std::{sync::atomic::AtomicI32, io::{Read, BufReader}, collections::{hash_map, HashMap}};
use std::{fs::File, io, io::Write, time::Duration, vec};
use base64::{Engine as _, engine::general_purpose};
use itertools::Itertools;

/// The index of the currently selected tab.
static INDEX: AtomicI32 = AtomicI32::new(0);
mod canvas;
mod draw;
mod svg;
mod widget;

/// The main function.
fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut svgs_hashmap: HashMap<String, Vec<String>> = hash_map::HashMap::new();
    svgs_hashmap.insert("MHV".to_string(),vec![r#"<path d="M 50.000 75.000 V 25.000 M 25.000 50.000 H 75.000" style="stroke: rgb(255, 0, 0); stroke-width: 1; fill: none;"/>"#.to_string(),]);
    svgs_hashmap.insert("L".to_string(), vec![r#"<path d="M 25.000 25.000 L 75.000 25.000 L 75.000 75.000 L 25.000 75.000 Z" style="stroke: rgb(255, 0, 0); stroke-width: 1; fill: none;"/>"#.to_string(),]);
    svgs_hashmap.insert("C1".to_string(), vec![r#"<path d="M 25.000 25.000 C 25.000 30.000 75.000 27.000 45.000 25.000" style="stroke: rgb(255, 0, 0); stroke-width: 1; fill: none;"/>"#.to_string(),]);
    svgs_hashmap.insert("C2".to_string(), vec![r#"<path d="M 25.000 25.000 C 25.000 30.000 75.000 27.000 45.000 50.000" style="stroke: rgb(255, 0, 0); stroke-width: 1; fill: none;"/>"#.to_string(),]);
    svgs_hashmap.insert("A".to_string(), vec![r#"<path d="M 25.000 25.000 A 10.000 10.000 90.000 0 0 50.000 50.000" style="stroke: rgb(255, 0, 0); stroke-width: 1; fill: none;"/>"#.to_string(),]);
    svgs_hashmap.insert("AF1".to_string(), vec![r#"<path d="M 25.000 25.000 A 10.000 10.000 90.000 1 0 50.000 50.000" style="stroke: rgb(255, 0, 0); stroke-width: 1; fill: none;"/>"#.to_string(),]);
    svgs_hashmap.insert("AF2".to_string(), vec![r#"<path d="M 25.000 25.000 A 10.000 10.000 90.000 0 1 50.000 50.000" style="stroke: rgb(255, 0, 0); stroke-width: 1; fill: none;"/>"#.to_string(),]);

    let mut data_hashmap: HashMap<String, String> = hash_map::HashMap::new();
    data_hashmap.insert("test.svg".to_string(), String::new()); 
    data_hashmap.insert("current.svg".to_string(), String::new());
    
    for key in data_hashmap.clone().keys().sorted() {
        let file = File::open(key.clone()).unwrap();
        //open file as bytes and encode to base64
        let mut reader = BufReader::new(file);
        let mut contents = String::new();
        reader.read_to_string(&mut contents).unwrap();
        let mut strings = Vec::new();
        for i in contents.split('\n') {
            strings.push(i.to_string());
        }
        // get strings between <svg> tags
        strings.remove(0);
        strings.remove(0);
        strings.pop();
        strings.pop();
        //panic!("{strings:?}");
        svgs_hashmap.insert(key.clone().strip_suffix(".svg").unwrap().to_string(), strings);
        *data_hashmap.get_mut(key).unwrap() = general_purpose::STANDARD_NO_PAD.encode(contents.as_bytes());
    }
    
    loop {
        for key in data_hashmap.clone().keys().sorted() {
            let file = File::open(key.clone()).unwrap();
            //open file as bytes and encode to base64
            let mut reader = BufReader::new(file);
            let mut contents = String::new();
            reader.read_to_string(&mut contents).unwrap();
            let check = general_purpose::STANDARD_NO_PAD.encode(contents.as_bytes());
            //panic!("{strings:?}");
            if check != *data_hashmap.get(key).unwrap() {
                svgs_hashmap.remove(key);
                let file = File::open(key.clone()).unwrap();
                //open file as bytes and encode to base64
                let mut reader = BufReader::new(file);
                let mut contents = String::new();
                reader.read_to_string(&mut contents).unwrap();
                let mut strings = Vec::new();
                for i in contents.split('\n') {
                    strings.push(i.to_string());
                }
                strings.remove(0);
                strings.remove(0);
                strings.pop();
                strings.pop();
                //panic!("{strings:?}");
                svgs_hashmap.insert(key.clone().strip_suffix(".svg").unwrap().to_string(),strings);

                *data_hashmap.get_mut(key).unwrap() = general_purpose::STANDARD_NO_PAD.encode(contents.as_bytes());

            } else {
                //println!("same");
            }
        }
        let mut tabs = Vec::new();

        // sort keys alphabetically 
        for i in svgs_hashmap.keys().sorted() {
            tabs.push(Spans::from(i.to_string()));
        }
        //let keys = svgs_hashmap.keys().sorted().cloned().collect::<Vec<String>>();
        
        read_input(&mut terminal, svgs_hashmap.clone())?;
        draw(&mut terminal, tabs.clone(), svgs_hashmap.clone())?;
        //println!("{}", tabs.clone().len());
    }
}

/// Using the api built into crossterm, we can read keyinputs.
pub fn read_input(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    hash_map: HashMap<String, Vec<String>>,
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
                            if INDEX.load(std::sync::atomic::Ordering::Relaxed) + 1 > hash_map.keys().len() as i32 - 1 {
                                INDEX.store(0, std::sync::atomic::Ordering::Relaxed);
                            } else {
                                INDEX.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                            }

                            return Ok(());
                        }
                        KeyCode::Left => {
                            if INDEX.load(std::sync::atomic::Ordering::Relaxed) - 1 < 0 {
                                INDEX.store(hash_map.keys().len() as i32 - 1, std::sync::atomic::Ordering::Relaxed);
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
                            let svg_string = hash_map.get(&hash_map.clone().keys().sorted().cloned().collect::<Vec<String>>()[INDEX.load(std::sync::atomic::Ordering::Relaxed) as usize]).unwrap().clone();
                            for i in &svg_string {
                                writeln!(
                                    f,
                                    "{i}"
                                ).unwrap();
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

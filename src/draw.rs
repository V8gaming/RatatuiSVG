use crate::canvas::canvas_draw;
use crate::svg::{render_svg, SvgPoints};
use crate::widget::{Svg, SvgDataset};
use crate::INDEX;
use ratatui::widgets::GraphType::Line as OtherLine;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    symbols::{self, DOT},
    text::Spans,
    widgets::{Block, Borders, Tabs},
    Frame, Terminal,
};
use regex::Regex;
use std::io::BufWriter;
use std::{
    collections::HashMap,
    fs::File,
    io::{self, Stdout, Write},
};
use itertools::Itertools;

/// render the tabs in a ratatui terminal
pub fn draw(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    tabs: Vec<Spans>,
    svgs: HashMap<String, Vec<String>>,
) -> Result<(), io::Error> {
    let draw = terminal.draw(|frame| {
        let terminal_rect = frame.size();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(terminal_rect);

        let left_tabs = Tabs::new(tabs)
            .block(
                Block::default()
                    .title("Graphical renderer")
                    .borders(Borders::all())
                    .style(Style::default().fg(Color::White)),
            )
            .highlight_style(Style::default().fg(Color::Yellow))
            .divider(DOT)
            .select(
                INDEX
                    .load(std::sync::atomic::Ordering::Relaxed)
                    .try_into()
                    .unwrap(),
            );
        frame.render_widget(left_tabs, chunks[0]);
        let keys = svgs.keys().sorted().cloned().collect::<Vec<String>>();
        let index: usize = INDEX
            .load(std::sync::atomic::Ordering::Relaxed)
            .try_into()
            .unwrap();
        draw_svg(
            svgs.get(&keys[index]).unwrap().clone(),
            frame,
            chunks[0],
            None,
        );
    });
    drop(draw);
    Ok(())
}

/// parse and render the svg to the terminal
pub fn draw_svg(
    strings: Vec<String>,
    frame: &mut Frame<CrosstermBackend<Stdout>>,
    layout: Rect,
    path: Option<&str>,
) {
    let width = layout.width as f64;
    let height = layout.height as f64;
    let ratio = width / height;
    /*
    <?xml version="1.0" encoding="utf-8"?>
    <svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">
        <path d="M 50.000 0.000 L 50.000 100.000" style="stroke: rgb(0, 0, 0); stroke-width: 1; fill: none;" />
        <path d="M 50.000 0.000 L 50.000 {}.000" style="stroke: rgb(0, 0, 0); stroke-width: 1; fill: none;" />
    </svg>
     */
    let header_1 = r#"<?xml version="1.0" encoding="utf-8"?>"#;
    let header_2 = r#"<svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">"#;
    let footer = r#"</svg>"#;

    let mut draw_svg = String::new();
    draw_svg.push_str(header_1);
    draw_svg.push_str(header_2);
    for i in strings {
        draw_svg.push_str(&i);
    }
    draw_svg.push_str(footer);
    //save svg to file

    let mut hash_map: HashMap<usize, SvgPoints> = HashMap::new();
    if path.is_some() {
        render_svg(path.unwrap().to_owned(), ratio, &mut hash_map)
    } else {
        render_svg(draw_svg, ratio, &mut hash_map);
    }
    let mut datasets = Vec::new();
    let re = Regex::new(r"stroke:\s*rgb\((\d+),\s*(\d+),\s*(\d+)\);").unwrap();
    let bg_re = Regex::new(r"fill:\s*rgb\((\d+),\s*(\d+),\s*(\d+)\);").unwrap();
    for i in hash_map.values() {
        let f = File::create("test.txt").unwrap();
        let mut f = BufWriter::new(f);
        for j in i.0.iter() {
            writeln!(f, "{j:?}").unwrap();
        }
        if i.2 {
            let bg_color = Color::Rgb(
                bg_re
                    .captures(&i.1)
                    .unwrap()
                    .get(1)
                    .unwrap()
                    .as_str()
                    .parse::<u8>()
                    .unwrap(),
                bg_re
                    .captures(&i.1)
                    .unwrap()
                    .get(1)
                    .unwrap()
                    .as_str()
                    .parse::<u8>()
                    .unwrap(),
                bg_re
                    .captures(&i.1)
                    .unwrap()
                    .get(1)
                    .unwrap()
                    .as_str()
                    .parse::<u8>()
                    .unwrap(),
            );
            let dataset = SvgDataset::default()
                .data(&i.0)
                .marker(symbols::Marker::Blocks(symbols::Blocks::FULL))
                .graph_type(OtherLine)
                .style(Style::default().fg(bg_color));
            datasets.push(dataset);
        } else {
            let color = Color::Rgb(
                re.captures(&i.1)
                    .unwrap()
                    .get(1)
                    .unwrap()
                    .as_str()
                    .parse::<u8>()
                    .unwrap(),
                re.captures(&i.1)
                    .unwrap()
                    .get(2)
                    .unwrap()
                    .as_str()
                    .parse::<u8>()
                    .unwrap(),
                re.captures(&i.1)
                    .unwrap()
                    .get(3)
                    .unwrap()
                    .as_str()
                    .parse::<u8>()
                    .unwrap(),
            );
            let dataset = SvgDataset::default()
                .data(&i.0)
                .marker(symbols::Marker::Braille)
                .graph_type(OtherLine)
                .style(Style::default().fg(color));
            datasets.push(dataset);
        }
    }
    canvas_draw(frame, layout, datasets)
}

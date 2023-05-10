use crate::INDEX;
use crate::svg::{render_svg, SvgPoints};
use regex::Regex;
use std::io::BufWriter;
use std::{
    collections::HashMap,
    io::{self, Stdout,Write},
    fs::File,

};
use tui::widgets::GraphType::Line as OtherLine;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    symbols::{self, DOT},
    text::Spans,
    widgets::{Axis, Block, Borders, Chart, Dataset, Tabs},
    Frame, Terminal,
};
/// render the tabs in a ratatui terminal
pub fn draw(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    tabs: Vec<Spans>,
    svgs: Vec<Vec<&str>>
) -> Result<(), io::Error> {
    let draw = terminal.draw(|frame| {
        let terminal_rect = frame.size();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(100),
                ]
                .as_ref(),
            )
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
        .select(INDEX.load(std::sync::atomic::Ordering::Relaxed).try_into().unwrap());
        frame.render_widget(left_tabs, chunks[0]);

        draw_svg(svgs[INDEX.load(std::sync::atomic::Ordering::Relaxed) as usize].clone(), frame, chunks[0], None);
    });
    drop(draw);
    Ok(())
}

/// parse and render the svg to the terminal
pub fn draw_svg(
    strings: Vec<&str>,
    frame: &mut Frame<CrosstermBackend<Stdout>>,
    layout: Rect,
    path: Option<&str>
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
        draw_svg.push_str(i);
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
            let dataset = Dataset::default()
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
            let dataset = Dataset::default()
                .data(&i.0)
                .marker(symbols::Marker::Braille)
                .graph_type(OtherLine)
                .style(Style::default().fg(color));
            datasets.push(dataset);
        }
    }
    let chart = Chart::new(datasets)
        .x_axis(
            Axis::default()
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, 100.0]),
        )
        .y_axis(
            Axis::default()
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, 100.0]),
        );
    frame.render_widget(chart, layout);

}

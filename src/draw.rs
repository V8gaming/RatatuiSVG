use crate::canvas::canvas_draw;
use crate::canvas::SvgDataset;
use crate::svg::{render_svg, SvgPoints};
use ratatui::widgets::GraphType::Line as OtherLine;
use ratatui::{
    backend::CrosstermBackend,
    layout::Rect,
    style::{Color, Style},
    symbols, Frame,
};
use regex::Regex;
use std::{collections::HashMap, io::Stdout};

/// parse and render the svg to the terminal
pub fn draw_svg(strings: Vec<String>, frame: &mut Frame<CrosstermBackend<Stdout>>, layout: Rect) {
    //let width = layout.width as f64;
    //let height = layout.height as f64;
    //let ratio = width / height;
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
    render_svg(draw_svg, &mut hash_map);

    let mut datasets = Vec::new();
    let re = Regex::new(r"stroke:\s*rgb\((\d+),\s*(\d+),\s*(\d+)\);").unwrap();
    let bg_re = Regex::new(r"fill:\s*rgb\((\d+),\s*(\d+),\s*(\d+)\);").unwrap();
    for i in hash_map.values() {
        /*         let f = File::create("test.txt").unwrap();
        let mut f = BufWriter::new(f);
        for j in i.0.iter() {
            writeln!(f, "{j:?}").unwrap();
        } */
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

use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufReader, Read, Stdout},
};

use base64::{engine::general_purpose, Engine as _};
use itertools::Itertools;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    symbols::{self, DOT},
    text::Spans,
    widgets::{
        canvas::{Canvas, Line, Points},
        Block, Borders, GraphType, Tabs,
    },
    Frame, Terminal,
};

use crate::{draw::draw_svg, INDEX};
/// A group of data points
#[derive(Debug, Clone)]
pub struct SvgDataset<'a> {
    /// A reference to the actual data
    data: &'a [(f64, f64, bool)],
    /// Symbol used for each points of this dataset
    marker: symbols::Marker,
    /// Determines graph type used for drawing points
    graph_type: GraphType,
    /// Style used to plot this dataset
    style: Style,
}

impl<'a> Default for SvgDataset<'a> {
    fn default() -> SvgDataset<'a> {
        SvgDataset {
            data: &[],
            marker: symbols::Marker::Dot,
            graph_type: GraphType::Scatter,
            style: Style::default(),
        }
    }
}

impl<'a> SvgDataset<'a> {
    pub fn data(mut self, data: &'a [(f64, f64, bool)]) -> SvgDataset<'a> {
        self.data = data;
        self
    }

    pub fn marker(mut self, marker: symbols::Marker) -> SvgDataset<'a> {
        self.marker = marker;
        self
    }

    pub fn graph_type(mut self, graph_type: GraphType) -> SvgDataset<'a> {
        self.graph_type = graph_type;
        self
    }

    pub fn style(mut self, style: Style) -> SvgDataset<'a> {
        self.style = style;
        self
    }
}

pub fn canvas_draw(
    frame: &mut Frame<CrosstermBackend<Stdout>>,
    layout: Rect,
    datasets: Vec<SvgDataset>,
) {
    let canvas = Canvas::default()
        .x_bounds([0.0, 100.0])
        .y_bounds([0.0, 100.0])
        .marker(ratatui::symbols::Marker::Braille)
        .paint(|ctx| {
            for dataset in datasets.clone() {
                ctx.layer();
                for data in dataset.data.windows(2) {
                    match dataset.graph_type {
                        GraphType::Line => {
                            if data[1].2 {
                                ctx.draw(&Line {
                                    color: dataset.style.fg.unwrap_or(Color::Reset),
                                    x1: data[0].0,
                                    y1: data[0].1,
                                    x2: data[1].0,
                                    y2: data[1].1,
                                });
                            }
                        }
                        GraphType::Scatter => {
                            ctx.draw(&Points {
                                color: dataset.style.fg.unwrap_or(Color::Reset),
                                coords: dataset.data,
                            });
                        }
                    }
                }
            }
        });
    frame.render_widget(canvas, layout);
}

#[derive(Clone)]
pub struct Svg {
    svgs: HashMap<String, Vec<String>>,
    files: HashMap<String, String>,
}

impl Default for Svg {
    fn default() -> Self {
        Self::new()
    }
}
impl Svg {
    pub fn new() -> Svg {
        Svg {
            svgs: HashMap::new(),
            files: HashMap::new(),
        }
    }
    pub fn run(&mut self, terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Svg {

        self.check();
        
        let mut tabs = Vec::new();

        for i in self.keys() {
            tabs.push(Spans::from(i.to_string()));
        }

        self.draw(terminal, tabs).ok();

        self.to_owned()

    }

    pub fn len(self) -> usize {
        self.svgs.len()
    }

    pub fn get(self, key: &str) -> Option<Vec<String>> {
        self.svgs.get(key).cloned()
    }
    pub fn import(&mut self, file: String, base64: String) -> &Svg {
        self.files.insert(file, base64);
        self
    }

    pub fn add(&mut self, key: String, svg_vec: Vec<String>) -> &Svg {
        self.svgs.insert(key, svg_vec);
        self
    }

    pub fn remove(&mut self, key: &str) -> &Svg {
        self.svgs.remove(key);
        self
    }

    pub fn keys(&self) -> Vec<String> {
        self.svgs.keys().sorted().cloned().collect::<Vec<String>>()
    }

    pub fn as_hashmap(&self) -> &HashMap<String, Vec<String>> {
        &self.svgs
    }
    pub fn check(&mut self) -> Svg {
        for file in self.files.keys().sorted().cloned().collect::<Vec<String>>() {
            let current_file = File::open(file.clone()).unwrap();
            //open file as bytes and encode to base64
            let mut reader = BufReader::new(current_file);
            let mut contents = String::new();
            reader.read_to_string(&mut contents).unwrap();
            let check = general_purpose::STANDARD_NO_PAD.encode(contents.as_bytes());
            //panic!("{strings:?}");
            if check != *self.files.get(&file).unwrap() {
                self.remove(&file);
                let current_file = File::open(file.clone()).unwrap();
                //open file as bytes and encode to base64
                let mut reader = BufReader::new(current_file);
                let mut contents = String::new();
                reader.read_to_string(&mut contents).unwrap();
                let mut strings = Vec::new();
                let tags = ["<path", "<rect", "<line"];
                for i in contents.split('\n') {
                    for tag in tags {
                        if i.starts_with(tag) {
                            strings.push(i.to_string());
                            break;
                        }
                    }
                }
                self.svgs
                    .insert(file.strip_suffix(".svg").unwrap().to_string(), strings);
                *self.files.get_mut(&file).unwrap() =
                    general_purpose::STANDARD_NO_PAD.encode(contents.as_bytes());
            } else {
                //println!("same");
            }
        }
        self.to_owned()
    }
    pub fn initialize(&mut self) -> Svg { 
        for file in self.files.keys().sorted().cloned().collect::<Vec<String>>() {
            let current_file = File::open(file.clone()).unwrap();
            //open file as bytes and encode to base64
            let mut reader = BufReader::new(current_file);
            let mut contents = String::new();
            reader.read_to_string(&mut contents).unwrap();
            let mut strings = Vec::new();
            let tags = ["<path", "<rect", "<line"];
            for i in contents.split('\n') {
                for tag in tags {
                    if i.starts_with(tag) {
                        strings.push(i.to_string());
                        break;
                    }
                }
            }
            self.svgs.insert(file.strip_suffix(".svg").unwrap().to_string(), strings);
            *self.files.get_mut(&file).unwrap() =
                general_purpose::STANDARD_NO_PAD.encode(contents.as_bytes());
        }
        self.to_owned()
    
    }
    pub fn draw(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
        tabs: Vec<Spans>,
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
            let keys = &self.svgs.keys().sorted().cloned().collect::<Vec<String>>();
            let index: usize = INDEX
                .load(std::sync::atomic::Ordering::Relaxed)
                .try_into()
                .unwrap();
            draw_svg(
                self.svgs.get(&keys[index]).unwrap().clone(),
                frame,
                chunks[0],
            );
        });
        drop(draw);
        Ok(())
    }
}
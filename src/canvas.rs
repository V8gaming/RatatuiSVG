use std::io::Stdout;

use ratatui::{
    style::Color,
    widgets::{canvas::{self, Canvas, Line, Map, Points}, Widget, GraphType}, 
    backend::CrosstermBackend, layout::Rect, Frame,
};

use crate::widget::SvgDataset;

pub fn canvas_draw(
    frame: &mut Frame<CrosstermBackend<Stdout>>,
    layout: Rect,
    datasets: Vec<SvgDataset>) {
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

                        },
                        GraphType::Scatter => {
                            ctx.draw(&Points {
                                color: dataset.style.fg.unwrap_or(Color::Reset),
                                coords: dataset.data,
                            });
                        },
                    }

                }
            }
        });
    frame.render_widget(canvas, layout);
}

use std::{collections::HashMap, fs};

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
/// A regex to match the fill attribute of an SVG path.
/// to get [x, y, x] from fill: rgb(x, y, z);
static ref FILL_RE: Regex = Regex::new(r"fill:\s*rgb\((\d+),\s*(\d+),\s*(\d+)\);").unwrap();}

/// A type to hold the points, style, and if filled of an SVG path.
pub type SvgPoints = (Vec<(f64, f64, bool)>, String, bool);

pub fn render_svg(svg: String, ratio: f64, hash_map: &mut HashMap<usize, SvgPoints>) {
    let mut f = String::new();
    if svg.ends_with(".svg") {
        f = fs::read_to_string(svg).unwrap();
    } else {
        f = svg;
    }
    let mut view_box = Vec::new();
    // read the whole file
    let parser = xml::reader::EventReader::from_str(f.as_str());
    let mut objects = Vec::new();
    for event in parser {
        match event.unwrap() {
            xml::reader::XmlEvent::StartElement {
                name,
                attributes,
                namespace: _,
            } => match name.local_name.as_str() {
                "svg" => {
                    let viewbox = &attributes[0].value;
                    let viewbox_data = viewbox.split(' ').collect::<Vec<&str>>();
                    let width = viewbox_data[2].parse::<f64>().unwrap();
                    let height = viewbox_data[3].parse::<f64>().unwrap();
                    view_box.push(width);
                    view_box.push(height);
                }
                "path" => {
                    let mut variables = ("".to_string(), "".to_string(), "".to_string());
                    for i in &attributes {
                        match i.name.local_name.as_str() {
                            "d" => {
                                variables.0 = i.value.to_owned();
                            }
                            "style" => {
                                variables.1 = i.value.to_owned();
                            }
                            "transform" => {
                                variables.2 = i.value.to_owned();
                            }

                            _ => {}
                        }
                    }
                    objects.push(variables);
                }
                "line" => {
                    let len = hash_map.keys().len();
                    let mut variables = (
                        "".to_string(),
                        "".to_string(),
                        "".to_string(),
                        "".to_string(),
                        "".to_string(),
                    );
                    for i in &attributes {
                        match i.name.local_name.as_str() {
                            "x1" => {
                                variables.0 = i.value.to_owned();
                            }
                            "y1" => {
                                variables.1 = i.value.to_owned();
                            }
                            "x2" => {
                                variables.2 = i.value.to_owned();
                            }
                            "y2" => {
                                variables.3 = i.value.to_owned();
                            }
                            "style" => {
                                variables.4 = i.value.to_owned();
                            }
                            _ => {}
                        }
                    }
                    hash_map.insert(
                        len + 1,
                        (
                            vec![
                                (
                                    variables.0.parse::<f64>().unwrap() / (view_box[0] / 100.0),
                                    100.0
                                        - variables.1.parse::<f64>().unwrap()
                                            / (view_box[1] / 100.0),
                                    true,
                                ),
                                (
                                    variables.2.parse::<f64>().unwrap() / (view_box[0] / 100.0),
                                    100.0
                                        - variables.3.parse::<f64>().unwrap()
                                            / (view_box[1] / 100.0),
                                    true,
                                ),
                            ],
                            variables.4,
                            false,
                        ),
                    );
                }
                "rect" => {
                    let len = hash_map.keys().len();
                    let mut variables = (
                        "".to_string(),
                        "".to_string(),
                        "".to_string(),
                        "".to_string(),
                        "".to_string(),
                    );
                    for i in &attributes {
                        match i.name.local_name.as_str() {
                            "x" => {
                                variables.0 = i.value.to_owned();
                            }
                            "y" => {
                                variables.1 = i.value.to_owned();
                            }
                            "width" => {
                                variables.2 = i.value.to_owned();
                            }
                            "height" => {
                                variables.3 = i.value.to_owned();
                            }
                            "style" => {
                                variables.4 = i.value.to_owned();
                            }
                            _ => {}
                        }
                    }
                    let x = variables.0.parse::<f64>().unwrap() / (view_box[0] / 100.0);
                    let y = variables.1.parse::<f64>().unwrap() / (view_box[1] / 100.0);
                    let width = variables.2.parse::<f64>().unwrap() / (view_box[0] / 100.0);
                    let height = variables.3.parse::<f64>().unwrap() / (view_box[1] / 100.0);
                    let style = variables.4.to_owned();
                    hash_map.insert(
                        len + 1,
                        (
                            vec![
                                (x, 100.0 - y, true),
                                (x + width, 100.0 - y, true),
                                (x + width, 100.0 - (y + height), true),
                                (x, 100.0 - (y + height), true),
                                (x, 100.0 - y, true),
                            ],
                            variables.4,
                            false,
                        ),
                    );
                    let mut fill: Vec<(f64, f64, bool)> = Vec::new();
                    if FILL_RE.is_match(&style) {
                        for i in (x as usize)..(x + width) as usize {
                            for j in (y as usize)..(y + height) as usize {
                                fill.push((i as f64, j as f64, true));
                            }
                        }
                    }
                    hash_map.insert(hash_map.len(), (fill, style, true));
                }
                _ => {}
            },
            xml::reader::XmlEvent::StartDocument {
                version: _,
                encoding: _,
                standalone: _,
            } => continue,
            xml::reader::XmlEvent::EndDocument => continue,
            xml::reader::XmlEvent::ProcessingInstruction { name: _, data: _ } => continue,
            xml::reader::XmlEvent::EndElement { name: _ } => continue,
            xml::reader::XmlEvent::CData(_) => continue,
            xml::reader::XmlEvent::Comment(_) => continue,
            xml::reader::XmlEvent::Characters(_) => continue,
            xml::reader::XmlEvent::Whitespace(_) => continue,
        }
    }

    for object in objects.iter().enumerate() {
        let points = draw_path(
            object.1 .0.to_owned(),
            view_box.clone(),
            ratio,
            &object.1 .2,
            &object.1 .1,
        );
        //mainstruct.data.log.push(format!("Points: {:?}", points));
        let style = object.1 .1.to_owned();

        hash_map.insert(hash_map.len() + 1, (points.0, style.clone(), false));
        if points.1.is_some() {
            hash_map.insert(hash_map.len() + 1 + 1, (points.1.unwrap(), style, true));
        }
    }
}
/// set of points for the path and filled points
pub type Points = (Vec<(f64, f64, bool)>, Option<Vec<(f64, f64, bool)>>);
fn draw_path(
    strings: String,
    view_box: Vec<f64>,
    ratio: f64,
    transform_str: &str,
    style: &str,
) -> Points {
    let mut points: Vec<(f64, f64, bool)> = Vec::new();

    let x_scale = view_box[0] / 100.0;
    let y_scale = view_box[1] / 100.0;

    let mut start: Option<(f64, f64, bool)> = None;
    let mut prev_point = (0.0, 0.0, true);
    let mut prev_command = "";
    let mut prev_match = "".to_string();
    let split = strings.split(' ').collect::<Vec<&str>>();
    let mut str_index = 0;
    let mut string_groups: Vec<Vec<&str>> = vec![Vec::new()];
    for i in split {
        let j: Vec<char> = i.chars().collect();
        if j.is_empty() {
            panic!("Invalid SVG -> Space at end of path");
        }
        if j[0].is_alphabetic() {
            if string_groups[str_index].is_empty() {
                string_groups[str_index].push(i);
            } else {
                str_index += 1;
                string_groups.push(Vec::new());
                string_groups[str_index].push(i);
            }
        } else {
            string_groups[str_index].push(i);
        }
    }
    for i in string_groups.into_iter() {
        //mainstruct.data.log.push(format!("i: {:?}", i));
        if i[0] == " " {
            continue;
        }
        if i[0] == "Z" {
            points.push(start.unwrap());
            continue;
        } else {
            //println!("{:?}", data);
            let command = i[0];

            match command {
                "M" => {
                    let x = i[1].parse::<f64>().unwrap();
                    let y = i[2].parse::<f64>().unwrap();
                    let mut x = x / x_scale;
                    let mut y = y / y_scale;
                    if Some(transform_str).is_none() {
                        continue;
                    } else {
                        let transformed_points = transform(x, y, transform_str, (x_scale, y_scale));
                        x = transformed_points.0;
                        y = transformed_points.1;
                    }

                    if start.is_none() {
                        //println!("Start: {:?}", (x, 100.0 - y));
                        start = Some((x, 100.0 - y, true));
                        points.push(start.unwrap());
                        prev_point = start.unwrap();
                    } else {
                        prev_point = (x, 100.0 - y, false);
                        points.push((x, 100.0 - y, false));
                    }

                    prev_command = command;
                    prev_match = format!("{}, {}, {}", command, x, 100.0 - y);
                }
                "L" => {
                    let x = i[1].parse::<f64>().unwrap();
                    let y = i[2].parse::<f64>().unwrap();
                    let mut x = x / x_scale;
                    let mut y = y / y_scale;
                    if Some(transform_str) == None {
                        continue;
                    } else {
                        let transformed_points = transform(x, y, transform_str, (x_scale, y_scale));
                        x = transformed_points.0;
                        y = transformed_points.1;
                    }
                    points.push((x, 100.0 - y, true));
                    prev_point = (x, 100.0 - y, true);
                    prev_command = command;
                    prev_match = format!("{}, {}, {}", command, x, 100.0 - y);
                }
                "Q" => {
                    // Quadratic Bezier Curve

                    let control_point_x = i[1].parse::<f64>().unwrap() / x_scale;
                    let control_point_y = 100.0 - i[2].parse::<f64>().unwrap() / y_scale;
                    let end_point_x = i[3].parse::<f64>().unwrap() / x_scale;
                    let end_point_y = 100.0 - i[4].parse::<f64>().unwrap() / y_scale;
                    for i in 0..100 {
                        let t = i as f64 / 100.0;
                        let point = quadratic_bezier_curve(
                            &prev_point,
                            &(control_point_x, control_point_y),
                            &(end_point_x, end_point_y),
                            t,
                            ratio,
                            Some(transform_str),
                            (x_scale, y_scale),
                        );
                        points.push(point);
                    }
                    prev_point = (end_point_x, end_point_y, true);
                    prev_command = command;
                    prev_match = format!(
                        "{command}, {control_point_x}, {control_point_y}, {end_point_x}, {end_point_y}"
                    );
                }
                "C" => {
                    // Cubic Bezier Curve
                    let control_point_1_x = i[1].parse::<f64>().unwrap() / x_scale;
                    let control_point_1_y = i[2].parse::<f64>().unwrap() / y_scale;
                    let control_point_2_x = i[3].parse::<f64>().unwrap() / x_scale;
                    let control_point_2_y = i[4].parse::<f64>().unwrap() / y_scale;
                    let end_point_x = i[5].parse::<f64>().unwrap() / x_scale;
                    let end_point_y = i[6].parse::<f64>().unwrap() / y_scale;
                    for i in 0..100 {
                        let t = i as f64 / 100.0;
                        let point = cubic_bezier_curve(
                            &prev_point,
                            &(control_point_1_x, control_point_1_y),
                            &(control_point_2_x, control_point_2_y),
                            &(end_point_x, end_point_y),
                            t,
                            Some(transform_str),
                            (x_scale, y_scale),
                        );
                        points.push(point);
                    }
                    prev_point = (end_point_x, end_point_y, true);
                    prev_command = command;
                    prev_match = format!(
                        "{command}, {control_point_1_x}, {control_point_1_y}, {control_point_2_x}, {control_point_2_y}, {end_point_x}, {end_point_y}"
                    );
                }
                "S" => {
                    let mut control_point_1_x = 0.0;
                    let mut control_point_1_y = 0.0;
                    let whitelist = vec!["C", "S"];
                    if whitelist.contains(&prev_command) {
                        // first control point is reflection of second control point on the previous command relative to the current point
                        let data = prev_match.split(", ").collect::<Vec<&str>>();
                        //mainstruct.data.log.push(format!("data: {:?}", data));
                        control_point_1_x =
                            prev_point.0 + (prev_point.0 - data[3].parse::<f64>().unwrap());
                        control_point_1_y =
                            prev_point.1 + (prev_point.1 - data[4].parse::<f64>().unwrap());
                    } else {
                        control_point_1_x = prev_point.0;
                        control_point_1_y = prev_point.1;
                    }
                    // Smooth Cubic Bezier Curve
                    let control_point_2_x = i[1].parse::<f64>().unwrap() / x_scale;
                    let control_point_2_y = i[2].parse::<f64>().unwrap() / y_scale;
                    let end_point_x = i[3].parse::<f64>().unwrap() / x_scale;
                    let end_point_y = i[4].parse::<f64>().unwrap() / y_scale;
                    for i in 0..100 {
                        let t = i as f64 / 100.0;
                        let point = cubic_bezier_curve(
                            &prev_point,
                            &(control_point_1_x, control_point_1_y),
                            &(control_point_2_x, control_point_2_y),
                            &(end_point_x, end_point_y),
                            t,
                            Some(transform_str),
                            (x_scale, y_scale),
                        );
                        points.push(point);
                    }
                    prev_point = (end_point_x, end_point_y, true);
                    prev_command = command;
                }
                "T" => {
                    // Smooth Quadratic Bezier Curve
                    let mut control_point_x = 0.0;
                    let mut control_point_y = 0.0;
                    let whitelist = vec!["Q", "T"];
                    if whitelist.contains(&prev_command) {
                        // first control point is reflection of second control point on the previous command relative to the current point
                        let data = prev_match.split(", ").collect::<Vec<&str>>();
                        control_point_x =
                            prev_point.0 + (prev_point.0 - data[1].parse::<f64>().unwrap());
                        control_point_y =
                            prev_point.1 + (prev_point.1 - data[2].parse::<f64>().unwrap());
                    } else {
                        control_point_x = prev_point.0;
                        control_point_y = prev_point.1;
                    }
                    let end_point_x = i[1].parse::<f64>().unwrap() / x_scale;
                    let end_point_y = i[2].parse::<f64>().unwrap() / y_scale;
                    for i in 0..100 {
                        let t = i as f64 / 100.0;
                        let point = quadratic_bezier_curve(
                            &prev_point,
                            &(control_point_x, control_point_y),
                            &(end_point_x, end_point_y),
                            t,
                            ratio,
                            Some(transform_str),
                            (x_scale, y_scale),
                        );
                        points.push(point);
                    }
                    prev_point = (end_point_x, end_point_y, true);
                    prev_command = command;
                }
                "A" => {
                    // Elliptical Arc
                    let rx = i[1].parse::<f64>().unwrap() / x_scale;
                    let ry = i[2].parse::<f64>().unwrap() / y_scale;
                    let x_axis_rotation = i[3].parse::<f64>().unwrap();
                    let large_arc_flag = i[4].parse::<i8>().unwrap() == 1;
                    let sweep_flag = i[5].parse::<i8>().unwrap() == 1;
                    let end_point_x = i[6].parse::<f64>().unwrap() / x_scale;
                    let end_point_y = i[7].parse::<f64>().unwrap() / y_scale;

                    let mut arc_points: Vec<(f64, f64, bool)> = elliptical_arc(
                        prev_point,
                        (rx, ry),
                        x_axis_rotation,
                        large_arc_flag,
                        sweep_flag,
                        (end_point_x, end_point_y),
                        100,
                    );
                    points.append(&mut arc_points);
                    prev_point = (end_point_x, end_point_y, true);
                    prev_command = command;
                }
                "H" => {
                    // Horizontal Line
                    let mut end_point_x = i[1].parse::<f64>().unwrap() / x_scale;
                    let mut end_point_y = prev_point.1;
                    if Some(transform_str) == None {
                        continue;
                    } else {
                        let transformed_points =
                            transform(end_point_x, end_point_y, transform_str, (x_scale, y_scale));
                        end_point_x = transformed_points.0;
                        end_point_y = transformed_points.1;
                    }
                    points.push((end_point_x, end_point_y, true));
                    prev_point = (end_point_x, end_point_y, true);
                    prev_command = command;
                }
                "V" => {
                    // Vertical Line
                    let mut end_point_x = prev_point.0;
                    let mut end_point_y = 100.0 - i[1].parse::<f64>().unwrap() / y_scale;
                    if Some(transform_str) == None {
                        continue;
                    } else {
                        let transformed_points =
                            transform(end_point_x, end_point_y, transform_str, (x_scale, y_scale));
                        end_point_x = transformed_points.0;
                        end_point_y = transformed_points.1;
                    }
                    //println!("{:?}", (end_point_x, end_point_y));
                    points.push((end_point_x, end_point_y, true));
                    prev_point = (end_point_x, end_point_y, true);
                    prev_command = command;
                }

                _ => {}
            }
        }

        //println!("{:?}", data);
    }
    // if "i" & "j" is within points than push to fill, i is 0 to 100,  j is 0 to 100
    let x_points: Vec<f64> = points.iter().map(|x| x.0).collect();
    let y_points: Vec<f64> = points.iter().map(|x| x.1).collect();
    if FILL_RE.is_match(style) {
        let x_min: usize = *x_points
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap() as usize
            + 1;
        let x_max: usize = *x_points
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap() as usize;
        let y_min: usize = *y_points
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap() as usize
            + 1;
        let y_max: usize = *y_points
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap() as usize;
        let mut fill: Vec<(f64, f64, bool)> = Vec::new();
        for i in x_min..x_max {
            for j in y_min..y_max {
                fill.push((i as f64, j as f64, true));
            }
        }
        //let mut f = File::create("fill.txt").unwrap();
        //writeln!(f, "{fill:?}").unwrap();

        (points, Some(fill))
    } else {
        (points, None)
    }
}

/* fn split_keep<'a>(r: &Regex, text: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    let mut last = 0;
    for (index, matched) in text.match_indices(r) {
        if last != index {
            result.push(&text[last..index]);
        }
        result.push(matched);
        last = index + matched.len();
    }
    if last < text.len() {
        result.push(&text[last..]);
    }
    result
} */
fn quadratic_bezier_curve(
    start: &(f64, f64, bool),
    control: &(f64, f64),
    end: &(f64, f64),
    t: f64,
    ratio: f64,
    transform_str: Option<&str>,
    scales: (f64, f64),
) -> (f64, f64, bool) {
    let x = (1.0 - t).powi(2) * start.0 + 2.0 * (1.0 - t) * t * control.0 + t.powi(2) * end.0;
    let y = (1.0 - t).powi(2) * start.1 + 2.0 * (1.0 - t) * t * control.1 + t.powi(2) * end.1;
    // if transform exists use transform, else return x,y
    if transform_str.is_none() {
        (x, y, true)
    } else {
        let (x, y) = transform(
            x,
            y,
            transform_str.expect("Unable to unwrap transform_str"),
            scales,
        );
        (x, y, true)
    }
}

fn cubic_bezier_curve(
    start: &(f64, f64, bool),
    control_1: &(f64, f64),
    control_2: &(f64, f64),
    end: &(f64, f64),
    t: f64,
    transform_str: Option<&str>,
    scales: (f64, f64),
) -> (f64, f64, bool) {
    let x = (1.0 - t).powi(3) * start.0
        + 3.0 * (1.0 - t).powi(2) * t * control_1.0
        + 3.0 * (1.0 - t) * t.powi(2) * control_2.0
        + t.powi(3) * end.0;
    let y = 100.0
        - ((1.0 - t).powi(3) * start.1
            + 3.0 * (1.0 - t).powi(2) * t * control_1.1
            + 3.0 * (1.0 - t) * t.powi(2) * control_2.1
            + t.powi(3) * end.1);
    // if transform exists use transform, else return x,y
    if transform_str == None {
        (x, y, true)
    } else {
        let (x, y) = transform(
            x,
            y,
            transform_str.expect("Unable to unwrap transform_str"),
            scales,
        );
        (x, y, true)
    }
}

fn elliptical_arc(
    start: (f64, f64, bool),
    radii: (f64, f64),
    x_axis_rotation: f64,
    large_arc_flag: bool,
    sweep_flag: bool,
    end: (f64, f64),
    num_points: usize,
) -> Vec<(f64, f64, bool)> {
    let (mut rx, mut ry) = radii;
    let (x1, y1) = (start.0, start.1);
    let (x2, y2) = end;

    let angle_rad = x_axis_rotation.to_radians();
    let cos_angle = angle_rad.cos();
    let sin_angle = angle_rad.sin();

    let x1_prime = cos_angle * (x1 - x2) / 2.0 + sin_angle * (y1 - y2) / 2.0;
    let y1_prime = -sin_angle * (x1 - x2) / 2.0 + cos_angle * (y1 - y2) / 2.0;

    let mut lambda = (x1_prime / rx).powi(2) + (y1_prime / ry).powi(2);
    if lambda > 1.0 {
        lambda = lambda.sqrt();
        rx *= lambda;
        ry *= lambda;
    }

    let (cx_prime, cy_prime) = {
        let sign = if large_arc_flag == sweep_flag {
            -1.0
        } else {
            1.0
        };
        let factor = ((rx.powi(2) * ry.powi(2)
            - rx.powi(2) * y1_prime.powi(2)
            - ry.powi(2) * x1_prime.powi(2))
            / (rx.powi(2) * y1_prime.powi(2) + ry.powi(2) * x1_prime.powi(2)))
        .sqrt();
        (
            sign * factor * rx * y1_prime / ry,
            -sign * factor * ry * x1_prime / rx,
        )
    };

    let (cx, cy) = (
        cos_angle * cx_prime - sin_angle * cy_prime + (x1 + x2) / 2.0,
        sin_angle * cx_prime + cos_angle * cy_prime + (y1 + y2) / 2.0,
    );

    let start_angle = ((y1_prime - cy_prime) / ry).atan2((x1_prime - cx_prime) / rx);
    let delta_angle = {
        let delta_angle =
            ((y1_prime * -1.0 - cy_prime) / ry).atan2((-x1_prime - cx_prime) / rx) - start_angle;
        let delta_angle = if delta_angle * (sweep_flag as i32 as f64 * 2.0 - 1.0) < 0.0 {
            delta_angle + 2.0 * std::f64::consts::PI
        } else {
            delta_angle
        };
        if sweep_flag {
            delta_angle
        } else {
            -delta_angle
        }
    };

    let mut points = Vec::with_capacity(num_points);
    for i in 0..=num_points {
        let t = i as f64 / num_points as f64;
        let angle = start_angle + t * delta_angle;
        let x = cx + rx * angle.cos();
        let y = cy + ry * angle.sin();
        let x_rotated = cos_angle * (x - cx) - sin_angle * (y - cy) + cx;
        let y_rotated = sin_angle * (x - cx) + cos_angle * (y - cy) + cy;
        points.push((x_rotated, y_rotated, true));
    }
    points
}
fn transform(x: f64, y: f64, transform: &str, scales: (f64, f64)) -> (f64, f64) {
    if transform.starts_with("matrix") {
        //matrix(0.963456, 0.267867, -0.267867, 0.963456, -67.327988, 51.384823)
        let data = transform.strip_prefix("matrix(").unwrap();
        let data = data.strip_suffix(')').unwrap();
        let data = data.split(", ").collect::<Vec<&str>>();
        let a = data[0].parse::<f64>().unwrap();
        let b = data[1].parse::<f64>().unwrap();
        let c = data[2].parse::<f64>().unwrap();
        let d = data[3].parse::<f64>().unwrap();
        let e = data[4].parse::<f64>().unwrap();
        let f = data[5].parse::<f64>().unwrap();
        let x = x * a + y * c + e;
        let y = x * b + y * d + f;
        (x, y)
    } else if transform.starts_with("translate") {
        let data = transform.strip_prefix("translate(").unwrap();
        let data = data.strip_suffix(')').unwrap();
        let data = data.split(", ").collect::<Vec<&str>>();
        let x = x + data[0].parse::<f64>().unwrap() / scales.0;
        let y = y + data[1].parse::<f64>().unwrap() / scales.1;
        return (x, y);
    } else if transform.starts_with("scale") {
        let data = transform.strip_prefix("scale(").unwrap();
        let data = data.strip_suffix(')').unwrap();
        let data = data.split(", ").collect::<Vec<&str>>();
        let x = x * data[0].parse::<f64>().unwrap();
        let y = y * data[1].parse::<f64>().unwrap();
        return (x, y);
    } else if transform.starts_with("rotate") {
        let data = transform.strip_prefix("rotate(").unwrap();
        let data = data.strip_suffix(')').unwrap();
        let data = data.split(' ').collect::<Vec<&str>>();
        let angle = data[0].parse::<f64>().unwrap();
        let angle_rad = angle.to_radians();
        let cos_a = angle_rad.cos();
        let sin_a = angle_rad.sin();
        let new_x = x * cos_a - y * sin_a;
        let new_y = x * sin_a + y * cos_a;

        return (new_x, new_y);
    } else if transform.starts_with("skewX") {
        let data = transform.strip_prefix("skewX(").unwrap();
        let data = data.strip_suffix(')').unwrap();
        let data = data.split(' ').collect::<Vec<&str>>();
        let angle = data[0].parse::<f64>().unwrap();
        let x = x + y * angle.tan();
        return (x, y);
    } else if transform.starts_with("skewY") {
        let data = transform.strip_prefix("skewY(").unwrap();
        let data = data.strip_suffix(')').unwrap();
        let data = data.split(' ').collect::<Vec<&str>>();
        let angle = data[0].parse::<f64>().unwrap();
        let y = y + x * angle.tan();
        return (x, y);
    } else {
        return (x, y);
    }
}

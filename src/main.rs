use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Point};
use core::f32::consts::PI;
use std::time::Duration;

const SCREEN_WIDTH: u32 = 1200;
const SCREEN_HEIGHT: u32 = 800;

const SPREAD: f32 = 1.1;
const LENGTH_CHANGE: f32 = 0.7;

struct LineSegment {
    start_point: Point,
    end_point: Point,
    length: f32,
    angle: f32,
    depth: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_left_child() {
        let start_line = LineSegment::new(
            (SCREEN_WIDTH / 2).try_into().unwrap(),
            750,
            (SCREEN_WIDTH / 2).try_into().unwrap(),
            450,
            PI / 2.0,
            0,
        );

        let (next_left, _next_right) = start_line.calculate_next_lines();

        assert_eq!(next_left.start_point, Point::new(400, 450));
        assert_eq!(next_left.end_point, Point::new(393, 421));
    }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Fractals in Rust!", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;

    let mut current_line_segments: Vec<LineSegment> = vec![LineSegment::new(
        (SCREEN_WIDTH / 2).try_into().unwrap(),
        750,
        (SCREEN_WIDTH / 2).try_into().unwrap(),
        450,
        PI / 2.0,
        1,
    )];

    let mut next_line_segments: Vec<LineSegment> = vec![];

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,

                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));

        for line in &current_line_segments {
            _ = canvas.draw_line(line.start_point, line.end_point);

            let (new_line_left, new_line_right) = line.calculate_next_lines();
            // line.print();
            // new_line_left.print();
            // new_line_right.print();

            if new_line_left.length > 0.0 || new_line_right.length > 0.0 {
                next_line_segments.push(new_line_left);
                next_line_segments.push(new_line_right);
            }
        }

        current_line_segments.clear();
        current_line_segments.append(&mut next_line_segments);
        next_line_segments.clear();

        // println!("DONE\n\n\n");

        canvas.present();
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32));
    }

    Ok(())
}

impl LineSegment {
    fn new(start_x: i32, start_y: i32, end_x: i32, end_y: i32, angle: f32, depth: i32) -> Self {
        Self {
            start_point: Point::new(start_x, start_y),
            end_point: Point::new(end_x, end_y),
            length:
                (((start_x - end_x) * (start_x - end_x) + (start_y - end_y) * (start_y - end_y))
                    as f32)
                    .sqrt(),
            angle: angle,
            depth: depth,
        }
    }

    fn print(&self) {
        println!(
            "{:?}, {:?}, {}, {}, {}",
            self.start_point,
            self.end_point,
            self.length,
            self.angle * 180.0 / PI,
            self.inclination() * 180.0 / PI
        );
    }

    fn inclination(&self) -> f32 {
        let dy = (self.end_point.y - self.start_point.y) as f32;
        let dx = (self.end_point.x - self.start_point.x) as f32;

        let angle = -dy.atan2(dx);

        if angle > 0.0 {
            return angle;
        } else {
            return angle + 2.0 * PI;
        }
    }

    fn calculate_next_lines(&self) -> (Self, Self) {

        let start_x = self.end_point.x;
        let start_y = self.end_point.y;
        let left_line_segment = Self::new(
            start_x,
            start_y,
            start_x + (self.length * LENGTH_CHANGE * (self.inclination() + self.angle / SPREAD).cos()) as i32,
            start_y - (self.length * LENGTH_CHANGE * (self.inclination() + self.angle / SPREAD).sin()) as i32,
            self.angle / SPREAD,
            self.depth + 1,
        );

        // println!("inside calculate_next_lines");
        // println!(
        //     "{}, {}",
        //     self.angle + self.angle / 2.0,
        //     (self.angle + self.angle / 2.0).sin()
        // );

        let right_line_segment = LineSegment::new(
            start_x,
            start_y,
            start_x + (self.length * LENGTH_CHANGE * (self.inclination() - self.angle / SPREAD).cos()) as i32,
            start_y - (self.length * LENGTH_CHANGE * (self.inclination() - self.angle / SPREAD).sin()) as i32,
            self.angle / SPREAD,
            self.depth + 1,
        );

        // println!(
        //     "{}, {}",
        //     self.angle - self.angle / 2.0,
        //     (self.angle - self.angle / 2.0).sin()
        // );

        return (left_line_segment, right_line_segment);
    }
}

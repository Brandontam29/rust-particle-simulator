use eframe::egui::Painter;
use epaint::{Color32, Pos2, Rect, RectShape, Shape, Stroke};

#[derive(Clone, Copy)]

pub struct Limits {
    pub top: f32,
    pub bottom: f32,
    pub left: f32,
    pub right: f32,
}

pub struct Canvas {
    position: Pos2,
    size: Pos2,
    color: Color32,
}

impl Canvas {
    pub fn new(position: Pos2, size: Pos2, color: Color32) -> Canvas {
        Canvas {
            position,
            size,
            color,
        }
    }

    pub fn paint(&self, painter: &Painter) {
        let rect = Shape::Rect(RectShape::new(
            Rect {
                min: self.position,
                max: self.position + self.size.to_vec2(),
            },
            0.0,
            self.color,
            Stroke {
                width: 0.0,
                color: Color32::WHITE,
            },
        ));

        painter.add(rect);
    }

    pub fn limits(&self) -> Limits {
        let new_position = self.position + self.size.to_vec2();

        Limits {
            top: self.position.y,
            left: self.position.x,
            bottom: new_position.y,
            right: new_position.y,
        }
    }
}

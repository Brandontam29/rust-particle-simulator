use eframe::egui::Painter;
use epaint::{CircleShape, Color32, Pos2, Shape, Stroke};

use crate::canvas::Limits;

#[derive(Debug, Clone)]
struct ParticleOriginal {
    position: Pos2,
    velocity: Pos2, // velocity per second
    mass: f32,
    diameter: f32,
    color: Color32,
}

pub struct Particle {
    original: ParticleOriginal,
    position: Pos2,
    velocity: Pos2, // velocity per second
    mass: f32,
    diameter: f32,
    color: Color32,
    limits: Limits,
}

impl Particle {
    pub fn new(
        position: Pos2,
        velocity: Pos2,
        mass: f32,
        diameter: f32,
        color: Color32,
        limits: Limits,
    ) -> Particle {
        Particle {
            original: ParticleOriginal {
                position,
                velocity,
                mass,
                diameter,
                color,
            },
            position,
            velocity,
            mass,
            diameter,
            color,
            limits,
        }
    }

    pub fn update(&mut self) {
        let new_position = self.position + self.velocity.to_vec2();

        if self.is_outside(new_position) {
            self.reset();
            return;
        }

        self.position = new_position;
    }

    fn reset(&mut self) {
        self.position = self.original.position;
        self.velocity = self.original.velocity;
        self.mass = self.original.mass;
        self.diameter = self.original.diameter;
        self.color = self.original.color;
    }

    fn is_outside(&self, position: Pos2) -> bool {
        let radius = self.diameter / 2.0;

        let left = position.x - radius;
        let top = position.y - radius;
        let bottom = position.y + radius;
        let right = position.x + radius;

        return left < self.limits.left
            || top < self.limits.top
            || right > self.limits.right
            || bottom > self.limits.bottom;
    }

    pub fn paint(&self, painter: &Painter) {
        let shape = Shape::Circle(CircleShape {
            center: self.position,
            radius: self.diameter / 2.0,
            fill: self.color,
            stroke: Stroke {
                width: 1.0,
                color: Color32::BLACK,
            },
        });

        painter.add(shape);
    }
}

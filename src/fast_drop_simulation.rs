use eframe::egui::Painter;
use epaint::{Color32, Pos2};

use crate::{
    canvas::{Canvas, Limits},
    particle::Particle,
};

fn generate_particles(position: Pos2, size: Pos2, n: usize, limits: &Limits) -> Vec<Particle> {
    let mut particles = Vec::with_capacity(n);

    for i in 0..n {
        let position = Pos2 {
            x: position.x + size.x / n as f32 * i as f32,
            y: position.y + 4.0,
        };
        let velocity = Pos2 { x: 0.0, y: 1.0 };
        let mass = 1.0;
        let diameter = 2.0;
        let color = Color32::BLACK;

        let particle = Particle::new(position, velocity, mass, diameter, color, limits.to_owned());

        particles.push(particle);
    }

    particles
}

pub struct FastDropSimulation {
    canvas: Canvas,
    particles: Vec<Particle>,
}

impl FastDropSimulation {
    pub fn new(position: Pos2, size: Pos2, n: usize) -> FastDropSimulation {
        let canvas = Canvas::new(position, size, Color32::WHITE);
        let limits = canvas.limits();
        let particles = generate_particles(position, size, n, &limits);

        FastDropSimulation { canvas, particles }
    }

    pub fn paint(&mut self, painter: &Painter) {
        self.canvas.paint(painter);
        for particle in &mut self.particles {
            particle.paint(painter);
        }
    }

    pub fn update(&mut self) {
        for particle in &mut self.particles {
            particle.update();
        }
    }
}

use crate::firework::explosion::Explosion;
use crate::firework::vector2d::Vector2D;
use femtovg::renderer::OpenGl;
use femtovg::{Canvas, Color, Paint, Path};

pub struct Sparkle {
    position: Vector2D,
    direction: Vector2D,
    color: Color,
}

impl Sparkle {
    pub fn new(position: Vector2D, direction: Vector2D, color: Color) -> Self {
        Sparkle {
            position,
            direction,
            color,
        }
    }

    pub fn update_and_draw(&mut self, delta_time: f32, gravity_factor: f32, life_time_01: f32, canvas: &mut Canvas<OpenGl>) {
        let old_position = self.position;
        let new_position = old_position + (self.direction * delta_time * 100.0 * life_time_01) - Vector2D::new(0.0, gravity_factor * delta_time);
        self.position = new_position;

        let mut path = Path::new();
        path.move_to(old_position.x, old_position.y);
        path.line_to(new_position.x, new_position.y);

        let color = Color::rgbf(self.color.r * life_time_01, self.color.g * life_time_01, self.color.b * life_time_01);

        //self.color.a = life_time_01;

        canvas.stroke_path(&mut path, Paint::color(color).with_line_width(life_time_01 * 1.2));

        /*
        let mut path = Path::new();
        path.rect(new_position.x, new_position.y, 1.0, 1.0);
        canvas.fill_path(&mut path, Paint::color(self.color));
         */
    }
}

use crate::firework::sparkle::Sparkle;
use crate::firework::vector2d::Vector2D;
use femtovg::renderer::OpenGl;
use femtovg::{Canvas, Color, CompositeOperation, ImageId, RenderTarget};
use rand::Rng;
use simple_error::SimpleError;
use std::cell::RefCell;
use std::f32::consts::TAU;
use std::rc::Rc;

const BEGIN_FADE_OUT_TIME: f32 = 1.0;

pub struct Explosion {
    lifetime: f32,
    time_left: f32,
    sparkles: Vec<Sparkle>,
    images: Rc<RefCell<Vec<ImageId>>>,
    image_index: usize,
    unused_images: Rc<RefCell<Vec<usize>>>,
    gravity_factor: f32,
}

impl Explosion {
    pub fn spawn(
        lifetime: f32,
        num_sparkles: usize,
        images: Rc<RefCell<Vec<ImageId>>>,
        unused_images: Rc<RefCell<Vec<usize>>>,
        explosions: Rc<RefCell<Vec<Explosion>>>,
        canvas: &mut Canvas<OpenGl>,
    ) -> Result<(), SimpleError> {
        let image_index = {
            let maybe_image = unused_images.borrow_mut().pop();
            match maybe_image {
                None => return Err(SimpleError::new("No unused image found")),
                Some(image) => image,
            }
        };

        let image = images.borrow()[image_index];

        canvas.set_render_target(RenderTarget::Image(image));

        canvas.clear_rect(
            0,
            0,
            canvas.width() as u32,
            canvas.height() as u32,
            Color::rgba(0, 0, 0, 0),
        );

        let mut rng = rand::thread_rng();

        let initial_position = Vector2D::new(
            rng.gen_range(50.0..canvas.width() - 50.0),
            rng.gen_range(50.0..canvas.height() - 50.0),
        );

        let color1 = Explosion::generate_color(&mut rng);
        let color2 = Explosion::generate_color(&mut rng);

        let sparkles = (0..num_sparkles)
            .map(|i| {
                let angle = rng.gen_range(0.0..TAU);
                let radius = rng.gen_range(0.5..1.0);
                let x = angle.cos() * radius;
                let y = angle.sin() * radius;

                Sparkle::new(
                    initial_position,
                    Vector2D::new(x, y),
                    if i % 2 == 0 { color1 } else { color2 },
                )
            })
            .collect();

        let mut explosions = explosions.borrow_mut();

        explosions.push(Explosion {
            lifetime,
            time_left: lifetime,
            sparkles,
            images,
            image_index,
            unused_images,
            gravity_factor: 0.0,
        });

        Ok(())
    }

    pub fn update(&mut self, delta_time: f32, canvas: &mut Canvas<OpenGl>) -> f32 {
        let image = self.get_image();

        canvas.set_render_target(RenderTarget::Image(image));
        canvas.global_composite_operation(CompositeOperation::Lighter);

        self.gravity_factor += 9.82 * delta_time;

        for sparkle in &mut self.sparkles {
            // Draw sparkles
            sparkle.update_and_draw(
                delta_time,
                self.gravity_factor,
                self.time_left / self.lifetime,
                canvas,
            );
        }

        canvas.flush();

        self.time_left -= delta_time;

        if self.time_left <= 0.0 {
            canvas.clear_rect(
                0,
                0,
                canvas.width() as u32,
                canvas.height() as u32,
                Color::rgba(0, 0, 0, 0),
            )
        }

        if self.time_left > BEGIN_FADE_OUT_TIME {
            1.0
        } else {
            self.time_left / BEGIN_FADE_OUT_TIME
        }
    }

    pub fn get_image(&mut self) -> ImageId {
        self.images.borrow()[self.image_index]
    }

    fn generate_color(rng: &mut impl Rng) -> Color {
        let r = rng.gen_range(0.0..1.0);
        let g = rng.gen_range(0.0..1.0);
        let b = rng.gen_range(0.0..1.0);

        Color::rgbf(r, g, b)
    }
}

impl Drop for Explosion {
    fn drop(&mut self) {
        self.unused_images.borrow_mut().push(self.image_index);
    }
}

impl PartialEq for Explosion {
    fn eq(&self, other: &Self) -> bool {
        self.image_index == other.image_index
    }
}

mod firework;

use crate::firework::explosion::Explosion;
use femtovg::renderer::OpenGl;
use femtovg::RenderTarget::Image;
use femtovg::{
    Canvas, Color, CompositeOperation, ImageFlags, Paint, Path, PixelFormat, RenderTarget,
};
use glutin::ContextBuilder;
use rand::Rng;
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;
use std::time::Instant;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    let mut rng = rand::thread_rng();

    let event_loop = EventLoop::new();

    let window_builder = WindowBuilder::new()
        .with_resizable(true)
        .with_title("Fireworks!");

    let window_context = ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(16)
        .build_windowed(window_builder, &event_loop)?;

    let window_context = unsafe { window_context.make_current().unwrap() };

    let renderer = OpenGl::new_from_glutin_context(&window_context)?;

    let mut canvas = Canvas::new(renderer)?;
    let size = window_context.window().inner_size();
    canvas.set_size(
        size.width,
        size.height,
        window_context.window().scale_factor() as f32,
    );

    let mut star_image = canvas.create_image_empty(
        canvas.width() as usize,
        canvas.height() as usize,
        PixelFormat::Rgb8,
        ImageFlags::empty(),
    )?;

    let mut star_paint = Paint::color(Color::black());

    let mut last_update = Instant::now();

    let mut box_x = 0.0;

    let mut create_image = || {
        canvas
            .create_image_empty(
                canvas.width() as usize,
                canvas.height() as usize,
                PixelFormat::Rgba8,
                ImageFlags::empty(),
            )
            .unwrap()
    };

    let images = Rc::new(RefCell::new(vec![]));
    let unused_images = Rc::new(RefCell::new(vec![]));

    {
        let mut images = images.borrow_mut();
        let mut unused_images = unused_images.borrow_mut();
        for i in 0..10 {
            images.push(create_image());
            unused_images.push(i);
        }
    }

    let explosions = Rc::new(RefCell::new(Vec::new()));

    let mut last_explosion_time = 0.0;
    let mut last_fps_report = 0.0;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        let window = window_context.window();

        match event {
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::Resized(size) => {
                    window_context.resize(*size);

                    if !(size.width == 0 && size.height == 0) {
                        canvas.set_size(size.width, size.height, window.scale_factor() as f32);

                        // Works:
                        canvas.delete_image(star_image);
                        star_image = canvas
                            .create_image_empty(
                                canvas.width() as usize,
                                canvas.height() as usize,
                                PixelFormat::Rgb8,
                                ImageFlags::empty(),
                            )
                            .unwrap();

                        // Does not work:
                        /*
                        canvas.realloc_image(
                            star_image,
                            size.width as usize,
                            size.height as usize,
                            PixelFormat::Rgb8,
                            ImageFlags::empty()
                        ).unwrap();
                         */

                        let mut images = images.borrow_mut();

                        for i in 0..images.len() {
                            //canvas.realloc_image(images[i], canvas.width() as usize, canvas.height() as usize, PixelFormat::Rgba8, ImageFlags::empty()).unwrap();
                            canvas.delete_image(images[i]);
                            images[i] = canvas
                                .create_image_empty(
                                    canvas.width() as usize,
                                    canvas.height() as usize,
                                    PixelFormat::Rgba8,
                                    ImageFlags::empty(),
                                )
                                .unwrap();
                        }

                        println!("Resize! w: {} h: {}", size.width, size.height);
                        star_paint = Paint::image(
                            star_image,
                            0.0,
                            0.0,
                            canvas.width(),
                            canvas.height(),
                            0.0,
                            1.0,
                        );

                        canvas.set_render_target(Image(star_image));

                        canvas.clear_rect(
                            0,
                            0,
                            canvas.width() as u32,
                            canvas.height() as u32,
                            Color::black(),
                        );

                        for _ in 0..1000 {
                            let x = rng.gen_range(0.0..(canvas.width()));
                            let y = rng.gen_range(0.0..(canvas.height()));

                            let mut path = Path::new();
                            path.rect(x, y, 1.0, 1.0);

                            let paint = Paint::color(Color::white());

                            canvas.fill_path(&mut path, paint);
                        }

                        canvas.set_render_target(RenderTarget::Screen);
                    }
                }
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => {}
            },
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                let now = Instant::now();
                let delta_time_duration = now.saturating_duration_since(last_update);
                let delta_time = delta_time_duration.as_secs_f32();
                last_update = now;

                last_explosion_time += delta_time;
                if last_explosion_time > 2.0 {
                    Explosion::spawn(
                        5.0,
                        160,
                        images.clone(),
                        unused_images.clone(),
                        explosions.clone(),
                        &mut canvas,
                    )
                    .unwrap();
                    last_explosion_time = 0.0;
                }

                last_fps_report += delta_time;
                if last_fps_report >= 1.0 {
                    println!("FPS: {}", 1.0 / delta_time);
                    last_fps_report = 0.0;
                }

                canvas.set_render_target(RenderTarget::Screen);
                canvas.global_composite_operation(CompositeOperation::SourceOver);

                canvas.clear_rect(
                    0,
                    0,
                    canvas.width() as u32,
                    canvas.height() as u32,
                    Color::white(),
                );

                let mut star_path = Path::new();
                star_path.rect(0.0, 0.0, canvas.width(), canvas.height());

                canvas.fill_path(&mut star_path, star_paint);

                let mut box_path = Path::new();
                box_path.rect(box_x, 20.0, 50.0, 50.0);

                canvas.fill_path(&mut box_path, Paint::color(Color::rgb(255, 0, 255)));

                box_x += delta_time * 100.0;
                if box_x >= canvas.width() {
                    box_x = 0.0;
                }

                let mut explosions_to_remove = vec![];
                {
                    let mut explosions = explosions.borrow_mut();
                    for i in 0..explosions.len() {
                        let alpha = explosions[i].update(delta_time, &mut canvas);

                        if alpha <= 0.0 {
                            explosions_to_remove.push(i);
                        }

                        canvas.set_render_target(RenderTarget::Screen);
                        canvas.global_composite_operation(CompositeOperation::Lighter);

                        let image = explosions[i].get_image();

                        let mut image_path = Path::new();
                        image_path.rect(0.0, 0.0, canvas.width(), canvas.height());
                        canvas.fill_path(
                            &mut image_path,
                            Paint::image(
                                image,
                                0.0,
                                0.0,
                                canvas.width(),
                                canvas.height(),
                                0.0,
                                alpha,
                            ),
                        );
                    }
                }

                for explosion in explosions_to_remove {
                    let mut explosions = explosions.borrow_mut();
                    explosions.remove(explosion);
                }

                canvas.global_composite_operation(CompositeOperation::SourceOver);

                canvas.flush();
                window_context.swap_buffers().unwrap();
            }
            Event::LoopDestroyed => *control_flow = ControlFlow::Exit,
            _ => {}
        }
    });
}

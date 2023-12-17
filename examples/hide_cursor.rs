#![allow(clippy::single_match)]

use simple_logger::SimpleLogger;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

#[path = "util/fill.rs"]
mod fill;

fn main() -> Result<(), impl std::error::Error> {
    SimpleLogger::new().init().unwrap();
    let event_loop = EventLoop::new().unwrap();

    let window = WindowBuilder::new().build(&event_loop).unwrap();
    window.set_title("Bus error on key press");

    event_loop.run(move |event, elwt| {
        if let Event::WindowEvent { event, .. } = event {
            match event {
                WindowEvent::KeyboardInput { event, .. } if event.state.is_pressed() => {
                    println!("Hiding cursor.");
                    window.set_cursor_visible(false);
                }
                WindowEvent::RedrawRequested => {
                    fill::fill_window(&window);
                }
                WindowEvent::CloseRequested => {
                    elwt.exit();
                }
                _ => (),
            }
        }
    })
}

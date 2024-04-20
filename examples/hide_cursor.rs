#![allow(clippy::single_match)]

use simple_logger::SimpleLogger;
use winit::{event_loop::EventLoop, window::WindowBuilder};

fn main() -> Result<(), impl std::error::Error> {
    SimpleLogger::new().init().unwrap();
    let event_loop = EventLoop::new().unwrap();

    let window = WindowBuilder::new().build(&event_loop).unwrap();
    window.set_title("Hide cursor");

    let mut count = 0..;
    event_loop.run(move |_event, _elwt| {
        eprint!("Hiding cursor...");
        window.set_cursor_visible(false);
        eprintln!("OK ({})", count.next().unwrap());
    })
}

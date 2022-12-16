use tracing::{span, Level};

use tracing_subscriber::util::SubscriberInitExt;
use winit::event_loop::{EventLoop, ControlFlow};

mod error;
mod window;

fn main() { 
    std::env::set_var("RUST_LOG", "info");
    tracing_subscriber::FmtSubscriber::new().init();

    let span = span!(Level::INFO, "Initialize");
    let _guard = span.enter();
    let window_request = crate::window::Window::new();
    
    let (_, event_loop) = window_request.unwrap();
    run(event_loop);
}

pub fn run(runnable: EventLoop<()>) {
    runnable.run(|event, _, control_flow| {

        match event {
            winit::event::Event::NewEvents(cause) => {
            },
            winit::event::Event::WindowEvent { window_id, event } => {
                match event {  
                    winit::event::WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    _ => (),
                }
            }
            _ => (),
        }
    });
}

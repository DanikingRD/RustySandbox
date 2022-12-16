use client::Client;
use error::RendererError;
use tracing::{span, warn, Level};

use tracing_subscriber::util::SubscriberInitExt;
use wgpu::{Surface, SurfaceError};
use winit::{
    event,
    event_loop::{ControlFlow, EventLoop},
};

mod client;
mod error;
mod renderer;
mod window;
fn main() {
    std::env::set_var("RUST_LOG", "info");
    tracing_subscriber::FmtSubscriber::new().init();

    let span = span!(Level::INFO, "Initialize");
    let _guard = span.enter();
    let (window, event_loop, renderer) = match crate::window::Window::new() {
        Ok(instance) => instance,
        Err(error) => panic!("Failed to create window!: {:?}", error),
    };
    let client = client::Client::init(window, renderer);
    run(event_loop, client);
}

pub fn run(runnable: EventLoop<()>, mut client: Client) {
    runnable.run(move |event, _, control_flow| match event {
        event::Event::NewEvents(cause) => {}
        event::Event::WindowEvent { window_id, event } => match event {
            winit::event::WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            _ => (),
        },
        event::Event::RedrawRequested(window_id) => {
            if window_id == client.window_id() {
                on_redraw_requested(&mut client)
                    .expect("Unrecoverable Error when preparing for next frame")
            }
        }
        _ => (),
    });
}

fn on_redraw_requested(client: &mut Client) -> Result<(), RendererError> {
    if let Err(e) = client.renderer_mut().start_render() {
        match e {
            // TODO: handle render errors
            RendererError::SurfaceError(e) => {
                warn!("{:?}", e);
                match e {
                    SurfaceError::Lost => todo!(),
                    SurfaceError::OutOfMemory => todo!(),
                    // All other errors should be resolved on the next frame
                    _ => return Ok(()),
                }
            }
            _ => return Err(e),
        }
    }
    Ok(())
}

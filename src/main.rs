use client::Client;
use error::RendererError;

use tracing::{span, warn, Level};
use tracing_subscriber::util::SubscriberInitExt;

use wgpu::SurfaceError;
use winit::{event, event_loop::EventLoop};

mod client;
mod error;
mod renderer;
mod window;

fn main() {
    std::env::set_var("RUST_LOG", "info");
    tracing_subscriber::FmtSubscriber::new().init();
    
    let span = span!(Level::INFO, "Initialize");
    
    let _guard = span.enter();
    let (window, event_loop) = match crate::window::Window::new() {
        Ok(instance) => instance,
        Err(error) => panic!("Failed to create window!: {:?}", error),
    };
    let client = client::Client::init(window);
    drop(_guard);
    run(event_loop, client);
}

pub fn run(runnable: EventLoop<()>, mut client: Client) {
    runnable.run(move |event, _, control_flow| match event {
        event::Event::WindowEvent { window_id, event } => {
            let span = tracing::span!(Level::INFO, "Window Events");
            let _guard = span.enter();

            if window_id == client.window_id() {
                client
                    .window_mut()
                    .handle_window_events(event, control_flow);
            }
        }
        event::Event::MainEventsCleared => {
            on_redraw_requested(&mut client)
                .expect("Unrecoverable Error when preparing for next frame");
        }
        _ => (),
    });
}

fn on_redraw_requested(client: &mut Client) -> Result<(), RendererError> {
    if let Err(e) = client.window_mut().renderer_mut().start_render() {
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

use client::Client;
use error::RendererError;

use tracing::{span, Level};
use tracing_subscriber::util::SubscriberInitExt;
use winit::{event, event_loop::EventLoop};

mod buffer;
mod camera;
mod client;
mod cube;
mod egui_instance;
mod error;
mod renderer;
mod vertex;
mod window;

fn main() {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    tracing_subscriber::FmtSubscriber::new().init();

    let span = span!(Level::INFO, "Initialize");

    let _guard = span.enter();
    let (window, event_loop, renderer) = match crate::window::Window::new() {
        Ok(instance) => instance,
        Err(error) => panic!("Failed to create window!: {:?}", error),
    };
    let client = client::Client::init(window, renderer);
    drop(_guard);
    run(event_loop, client);
}

pub fn run(runnable: EventLoop<()>, mut client: Client) {
    runnable.run(move |event, _, control_flow| {
        client.gui.handle_event(&event);
        match event {
            event::Event::WindowEvent { window_id, event } => {
                let span = tracing::span!(Level::INFO, "Window Events");

                let _guard = span.enter();

                if window_id == client.window_id() {
                    client
                        .window
                        .handle_window_events(&event, control_flow, &mut client.renderer);

                    client.update(&event);
                }
            }
            event::Event::MainEventsCleared => {
                client.window.winit().request_redraw();
            }
            event::Event::RedrawRequested(..) => {
                on_redraw_requested(&mut client)
                    .expect("Unrecoverable Error when preparing for next frame");
            }
            _ => (),
        }
    });
}

fn on_redraw_requested(client: &mut Client) -> Result<(), RendererError> {
    let span = span!(Level::INFO, "Render");
    let _guard = span.enter();
    client.render().unwrap();
    Ok(())
}

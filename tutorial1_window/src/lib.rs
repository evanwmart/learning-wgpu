use winit::{
    event::*,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::WindowBuilder,
};

pub fn run() {
    env_logger::init(); // Initialize logging for error tracking
    let event_loop = EventLoop::new().unwrap(); // Create a new event loop
    let window = WindowBuilder::new().build(&event_loop).unwrap(); // Build the window

    // Start the event loop
    event_loop.run(move |event, control_flow| match event {
        Event::WindowEvent { ref event, window_id }
            if window_id == window.id() => match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                event: KeyEvent {
                    state: ElementState::Pressed,
                    physical_key: PhysicalKey::Code(KeyCode::Escape),
                    ..
                },
                ..
            } => control_flow.exit(), // Close window if "Escape" is pressed
            _ => {}
        },
        _ => {}
    });
}

use wry::application::event::{Event, StartCause, WindowEvent};
use wry::application::event_loop::{ControlFlow, EventLoop};

#[repr(C)]
pub struct EventProp {}

#[no_mangle]
extern fn createEventLoop() -> Box<EventLoop<()>> {
    EventLoop::new().into()
}

/// Runs the WebView event loop
#[no_mangle]
extern fn eventLoopRun(event_loop: Box<EventLoop<()>>, init: extern fn() -> ()) {
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::NewEvents(StartCause::Init) => init(),
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {}
        }
    });
}

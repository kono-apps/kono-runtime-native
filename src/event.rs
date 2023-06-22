use wry::application::event_loop::EventLoop;

#[no_mangle]
extern fn createEventLoop() -> Box<EventLoop<()>> {
    EventLoop::new().into()
}
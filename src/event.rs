use wry::application::event_loop::EventLoop;

#[no_mangle]
extern fn createEventLoop() -> Box<EventLoop<()>> {
    EventLoop::new().into()
}

#[no_mangle]
extern fn dropEventLoop(_: Box<EventLoop<()>>) {}

pub enum KonoEvent {}
use std::ffi::c_char;
use wry::application::dpi::PhysicalSize;
use wry::application::event_loop::EventLoop as WryEventLoop;
use wry::application::window::{Fullscreen, Window as WryWindow, WindowBuilder};

use crate::to_rust_string;

#[repr(C)]
pub struct Window {
    window: WryWindow,
}

#[repr(C)]
pub struct EventLoop(WryEventLoop<()>);

#[no_mangle]
extern fn createEventLoop() -> Box<EventLoop> {
    Box::new(EventLoop(WryEventLoop::new()))
}

#[no_mangle]
extern fn dropEventLoop(_: Box<EventLoop>) {
    // Do nothing, Rust will drop it for us
}

#[no_mangle]
extern fn createWindow(
    event_loop: Box<EventLoop>,
    title: *const c_char,
    full_screen: bool,
    resizable: bool,
    width: i32,
    height: i32,
    maximized: bool,
) -> Box<Window> {
    let window = WindowBuilder::new()
        .with_inner_size(PhysicalSize { width, height })
        .with_title(to_rust_string(title))
        .with_fullscreen(
            if full_screen {
                Some(Fullscreen::Borderless(None))
            } else {
                None
            }
        )
        .with_maximized(maximized)
        .with_resizable(resizable)
        .build(&event_loop.0)
        .expect("failed to create window");
    return Box::new(Window { window });
}

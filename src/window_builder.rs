use std::ffi::c_char;
use wry::application::dpi::PhysicalSize;
use wry::application::event_loop::EventLoop;
use wry::application::window::{Window, WindowBuilder};
use wry::application::window::Fullscreen::Borderless;
use crate::to_rust_string;

#[no_mangle]
extern fn createWindowBuilder() -> Box<WindowBuilder> {
    WindowBuilder::new().into()
}

#[no_mangle]
extern fn windowBuilderSetTitle(window: Box<WindowBuilder>, title: *const c_char) -> Box<WindowBuilder> {
    window.with_title(to_rust_string(title)).into()
}

#[no_mangle]
extern fn windowBuilderSetFullScreen(window: Box<WindowBuilder>, full_screen: bool) -> Box<WindowBuilder> {
    window.with_fullscreen(if full_screen {
        Some(Borderless(None))
    } else {
        None
    }).into()
}

#[no_mangle]
extern fn windowBuilderSetSize(window: Box<WindowBuilder>, width: i32, height: i32) -> Box<WindowBuilder> {
    window.with_inner_size(PhysicalSize { width, height }).into()
}

#[no_mangle]
extern fn windowBuilderSetMaximized(window: Box<WindowBuilder>, maximized: bool) -> Box<WindowBuilder> {
    window.with_maximized(maximized).into()
}

#[no_mangle]
extern fn windowBuilderSetMaximizable(window: Box<WindowBuilder>, maximizable: bool) -> Box<WindowBuilder> {
    window.with_maximizable(maximizable).into()
}

#[no_mangle]
extern fn windowBuilderSetClosable(window: Box<WindowBuilder>, closable: bool) -> Box<WindowBuilder> {
    window.with_closable(closable).into()
}

#[no_mangle]
extern fn windowBuilderSetResizable(window: Box<WindowBuilder>, resizable: bool) -> Box<WindowBuilder> {
    window.with_resizable(resizable).into()
}

#[no_mangle]
extern fn windowBuild(window: Box<WindowBuilder>, event_loop: &EventLoop<()>) -> Box<Window> {
    window.build(&event_loop)
        .unwrap()
        .into()
}


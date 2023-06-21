use std::borrow::Cow;
use std::env;
use std::ffi::c_char;
use wry::application::event::{Event, StartCause, WindowEvent};
use wry::application::event_loop::{ControlFlow, EventLoop};

use wry::application::window::Window;
use wry::http::header::CONTENT_TYPE;
use wry::http::Response;
use wry::webview::{WebContext, WebView, WebViewBuilder};
use crate::asset::Asset;

use crate::{to_rust_bytes, to_rust_string};

/// Creates a new WebViewBuilder.
///
/// Since all setter methods in a WebViewBuilder take ownership
/// of self, we have to take a pointer and return it to avoid
/// Rust dropping it and causing problems.
#[no_mangle]
extern fn createWebViewBuilder(window: Box<Window>) -> Box<WebViewBuilder<'static>> {
    WebViewBuilder::new(*window).expect("failed to create webview builder").into()
}

/// Sets the URL of the WebView
#[no_mangle]
extern fn webViewSetURL(webview_builder: Box<WebViewBuilder>, url: *const c_char) -> Box<WebViewBuilder> {
    let url = to_rust_string(url);

    webview_builder.with_url(&url)
        .expect(&*format!("failed to set URL to '{}'", url))
        .into()
}

/// Sets whether the webview allows dev-tools or not, such
/// as inspect element
#[no_mangle]
extern fn webViewSetDevTools(webview_builder: Box<WebViewBuilder>, dev_tools: bool) -> Box<WebViewBuilder> {
    webview_builder.with_devtools(dev_tools).into()
}

/// Adds a custom protocol for serving files
#[no_mangle]
#[cfg(windows)]
extern fn webViewAddCustomProtocol(
    webview_builder: Box<WebViewBuilder>,
    name: *const c_char,
    handler: extern "stdcall" fn(&mut Asset) -> &Asset,
) -> Box<WebViewBuilder> {
    let name = to_rust_string(name);
    webview_builder.with_custom_protocol(name, move |request| {
        let path = request.uri().path();
        let mut asset = Asset::new(path);
        let asset = handler(&mut asset);
        let content: Cow<[u8]> = to_rust_bytes(asset.content()).into();
        let mime_type = to_rust_string(asset.mime_type());
        Response::builder()
            .header(CONTENT_TYPE, mime_type)
            .body(content)
            .map_err(Into::into)
    }).into()
}

/// Adds a custom protocol for serving files
#[no_mangle]
#[cfg(not(windows))]
extern fn webViewAddCustomProtocol(
    webview_builder: Box<WebViewBuilder>,
    name: *const c_char,
    handler: extern fn(&mut Asset) -> &Asset,
) -> Box<WebViewBuilder> {
    let name = to_rust_string(name);
    webview_builder.with_custom_protocol(name, move |request| {
        let path = request.uri().path();
        let mut asset = Asset::new(path);
        let asset = handler(&mut asset);
        let content: Cow<[u8]> = to_rust_bytes(asset.content()).into();
        let mime_type = to_rust_string(asset.mime_type());
        Response::builder()
            .header(CONTENT_TYPE, mime_type)
            .body(content)
            .map_err(Into::into)
    }).into()
}

/// Finalizes the WebViewBuilder into a WebView
#[no_mangle]
extern fn webViewBuild(webview_builder: Box<WebViewBuilder>) -> Box<WebView> {
    let current_dir = env::current_dir().expect("failed to get running directory");
    return webview_builder.with_web_context(&mut WebContext::new(Some(current_dir)))
        .build()
        .expect("failed to create webview")
        .into();
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

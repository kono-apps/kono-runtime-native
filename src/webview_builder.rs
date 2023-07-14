use std::ffi::c_char;
use std::path::PathBuf;

use wry::application::window::Window;
use wry::http::header::CONTENT_TYPE;
use wry::http::Response;
use wry::webview::{WebContext, WebView, WebViewBuilder};
use crate::asset::Asset;

use crate::{to_java_string, to_rust_string};

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

/// Sets whether the webview allows dev-tools or not, such
/// as inspect element
#[no_mangle]
extern fn webViewAddInitScript(webview_builder: Box<WebViewBuilder>, script: *const c_char) -> Box<WebViewBuilder> {
    webview_builder.with_initialization_script(&to_rust_string(script)).into()
}

/// Sets whether the webview allows dev-tools or not, such
/// as inspect element
#[no_mangle]
extern fn webViewSetHTML(webview_builder: Box<WebViewBuilder>, html: *const c_char) -> Box<WebViewBuilder> {
    webview_builder.with_html(to_rust_string(html)).unwrap().into()
}

/// Sets whether the webview allows dev-tools or not, such
/// as inspect element
#[no_mangle]
extern fn webViewAddIPCHandler(
    webview_builder: Box<WebViewBuilder>,
    handler: extern fn(*const c_char) -> (),
) -> Box<WebViewBuilder> {
    webview_builder.with_ipc_handler(move |_: &Window, req: String| {
        let req = to_java_string(&req);
        handler(req)
    }).into()
}

/// Adds a custom protocol for serving files
#[no_mangle]
#[cfg(windows)]
extern fn webViewAddCustomProtocol(
    webview_builder: Box<WebViewBuilder>,
    name: *const c_char,
    handler: extern fn(*const c_char) -> &'static Asset,
) -> Box<WebViewBuilder> {
    let name = to_rust_string(name);
    webview_builder.with_custom_protocol(name, move |request|  {
        let path = request.uri().path();
        let asset = handler(to_java_string(path));
        let content = asset.content().into();
        Response::builder()
            .header(CONTENT_TYPE, asset.mime_type())
            .body(content)
            .map_err(Into::into)
    }).into()
}

/// Finalizes the WebViewBuilder into a WebView
#[no_mangle]
extern fn webViewBuild(webview_builder: Box<WebViewBuilder>, running_dir: *const c_char) -> Box<WebView> {
    let running_dir = PathBuf::from(to_rust_string(running_dir));
    return webview_builder.with_web_context(&mut WebContext::new(Some(running_dir)))
        .build()
        .expect("failed to create webview")
        .into();
}

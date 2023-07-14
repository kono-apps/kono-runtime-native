use std::ffi::c_char;
use wry::webview::{Theme, WebView, WebViewBuilder, WebViewBuilderExtWindows};
use crate::{to_java_string, to_rust_string};

#[no_mangle]
extern fn webViewEval(webview: &WebView, script: *const c_char) {
    webview.evaluate_script(&to_rust_string(script))
        .unwrap()
}

#[no_mangle]
extern fn webViewEvalWithCallback(
    webview: &WebView,
    script: *const c_char,
    callback: extern fn(*const c_char) -> (),
) {
    webview.evaluate_script_with_callback(&to_rust_string(script), move |res| {
        let j_result = to_java_string(&res);
        callback(j_result)
    }).unwrap();
}

#[no_mangle]
extern fn webViewBuilderSetTheme(
    webview: Box<WebViewBuilder>,
    theme: u8
) -> Box<WebViewBuilder> {
    let theme = match theme {
        0 => Theme::Dark,
        1 => Theme::Light,
        2 => Theme::Auto,
        v => panic!("Invalid theme: {v}. Expected 0 (dark), 1 (light), or 2 (auto)")
    };
    webview.with_theme(theme).into()
}
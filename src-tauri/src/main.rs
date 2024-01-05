// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use webview2_com::Microsoft::Web::WebView2::Win32;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn login(handle: tauri::AppHandle) -> bool {
    // 1. 创建窗口(使用tauri::WindowBuilder)
    // 2. 打开https://id.ouc.edu.cn/
    // 3. 等待登录成功
    // 4. 保存登录信息
    // 5. 关闭窗口
    // 6. 返回登录是否成功

    let _login_window: tauri::Window = tauri::WindowBuilder::new(
        &handle,
        "login",
        tauri::WindowUrl::External("https://id.ouc.edu.cn/".parse().unwrap()),
    )
    .title("信息门户登录")
    .user_agent("Mozilla/5.0 (X11; Linux x86_64; rv:85.0) Gecko/20100101 Firefox/85.0")
    .inner_size(400.0, 800.0)
    .build()
    .unwrap();

    // 使用_login_window.url()获取当前窗口的url，并判断是否登录成功
    // 如果登录成功，使用_login_window.close()关闭窗口
    // 如果登录失败，使用_login_window.eval()执行js代码，提示用户登录失败
    let mut _url = _login_window.url();
    while !_url
        .to_string()
        .starts_with("https://id.ouc.edu.cn/api/uia/index")
    {
        _url = _login_window.url();
    }
    let _ = _login_window.with_webview(|webview| {
        #[cfg(target_os = "linux")]
        {
            // see https://docs.rs/webkit2gtk/0.18.2/webkit2gtk/struct.WebView.html
            // and https://docs.rs/webkit2gtk/0.18.2/webkit2gtk/trait.WebViewExt.html
            use webkit2gtk::gio;
            use webkit2gtk::traits::{CookieManagerExt, WebContextExt, WebViewExt};
            webview.inner().set_zoom_level(0.75);
            // 获取cookie
            let cookie_manager = webview.inner().context().unwrap().cookie_manager().unwrap();
            let callback = |result: Result<Vec<soup::Cookie>, webkit2gtk::Error>| match result {
                Ok(cookies) => {
                    println!("Cookies received:");
                    for mut cookie in cookies {
                        println!(
                            "Name: {}, Value: {}",
                            cookie.name().unwrap(),
                            cookie.value().unwrap()
                        );
                    }
                }
                Err(err) => {
                    eprintln!("Error retrieving cookies: {}", err);
                }
            };
            let _ = cookie_manager.cookies(
                "https://id.ouc.edu.cn",
                Some(&gio::Cancellable::new()),
                callback,
            );
        }

        #[cfg(windows)]
        unsafe {
            // see https://docs.rs/webview2-com/0.19.1/webview2_com/Microsoft/Web/WebView2/Win32/struct.ICoreWebView2Controller.html
            use webview2_com::Microsoft::Web::WebView2::Win32::{
                ICoreWebView2, ICoreWebView2Controller, ICoreWebView2CookieManager,
            };
            let controller: ICoreWebView2Controller = webview.controller();
            controller.SetZoomFactor(0.75).unwrap();
            // 获取cookie
            let core_webview2: ICoreWebView2 = controller.CoreWebView2().unwrap();

            let tmp = core_webview2;
        }

        #[cfg(target_os = "macos")]
        unsafe {
            let () = msg_send![webview.inner(), setPageZoom: 0.75];
        }
    });

    // _login_window.close().unwrap();
    true
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![login])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

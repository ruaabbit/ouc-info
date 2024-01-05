// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub struct Cookie {
    pub name: String,
    pub value: String,
    pub domain: String,
    pub path: String,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn login_id_ouc_edu_cn(handle: tauri::AppHandle) -> bool {
    let (done_tx, done_rx) = oneshot::channel::<Vec<Cookie>>();

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
    _login_window
        .with_webview(move |webview| {
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
                        let mut cookie_str = vec![];

                        println!("Cookies received:");
                        for mut cookie in cookies {
                            let mut name = cookie.name().unwrap();
                            let mut value = cookie.value().unwrap();
                            let mut domain = cookie.domain().unwrap();
                            let mut path = cookie.path().unwrap();

                            cookie_str.push(Cookie {
                                name: take_pwstr(name),
                                value: take_pwstr(value),
                                domain: take_pwstr(domain),
                                path: take_pwstr(path),
                            });
                        }
                        done_tx.send(cookie_str).unwrap();
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
                use webview2_com::take_pwstr;
                use webview2_com::GetCookiesCompletedHandler;
                use webview2_com::Microsoft::Web::WebView2::Win32::{
                    ICoreWebView2, ICoreWebView2Controller, ICoreWebView2Cookie,
                    ICoreWebView2CookieManager, ICoreWebView2_2,
                };
                use windows::core::Interface;
                use windows::core::HSTRING;
                use windows::core::PWSTR;

                let controller: ICoreWebView2Controller = webview.controller();
                controller.SetZoomFactor(0.75).unwrap();
                // get ICoreWebView2
                let core_webview2: ICoreWebView2 = controller.CoreWebView2().unwrap();
                let core_webview2_2: ICoreWebView2_2 = core_webview2.cast().unwrap();
                let cookie_manager: ICoreWebView2CookieManager =
                    core_webview2_2.CookieManager().unwrap();
                let uri = HSTRING::from("");

                GetCookiesCompletedHandler::wait_for_async_operation(
                    Box::new(move |handler| {
                        cookie_manager.GetCookies(&uri, &handler)?;
                        Ok(())
                    }),
                    Box::new(move |hresult, list| {
                        hresult?;
                        match list {
                            Some(list) => {
                                let mut count: u32 = 0;
                                list.Count(&mut count)?;
                                // tracing::info!("count: {}", count);
                                let mut cookie_str = vec![];
                                // let mut session_id = "".to_string();
                                for i in 0..count {
                                    let cookie: ICoreWebView2Cookie = list.GetValueAtIndex(i)?;
                                    let mut name = PWSTR::null();
                                    let mut value = PWSTR::null();
                                    let mut domain = PWSTR::null();
                                    let mut path = PWSTR::null();
                                    cookie.Name(&mut name)?;
                                    cookie.Value(&mut value)?;
                                    cookie.Domain(&mut domain)?;
                                    cookie.Path(&mut path)?;

                                    cookie_str.push(Cookie {
                                        name: take_pwstr(name),
                                        value: take_pwstr(value),
                                        domain: take_pwstr(domain),
                                        path: take_pwstr(path),
                                    });
                                }
                                // 输出 cookie_str
                                println!("Cookies received:");
                                for cookie in cookie_str.iter() {
                                    println!(
                                        "Name: {}, Value: {}, Domain: {}, Path: {}",
                                        cookie.name, cookie.value, cookie.domain, cookie.path
                                    );
                                }

                                done_tx.send(cookie_str).unwrap();
                            }
                            None => {
                                // tracing::info!("list is None");
                            }
                        };
                        Ok(())
                    }),
                )
                .unwrap();
            }

            #[cfg(target_os = "macos")]
            unsafe {
                let () = msg_send![webview.inner(), setPageZoom: 0.75];
            }
        })
        .unwrap();

    // _login_window.close().unwrap();

    // let cookies = done_rx.await.unwrap();

    true
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![login_id_ouc_edu_cn])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

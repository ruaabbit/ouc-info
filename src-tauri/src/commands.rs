use crate::utils;

#[tauri::command]
pub async fn login_id_ouc_edu_cn(handle: tauri::AppHandle) -> Result<Vec<utils::Cookie>, String> {
    let login_window: tauri::Window = tauri::WindowBuilder::new(
        &handle,
        "login",
        tauri::WindowUrl::External("https://id.ouc.edu.cn/".parse().unwrap()),
    )
    .title("信息门户登录")
    .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36 Edg/120.0.0.0 ITStudio")
    .inner_size(400., 800.)
    .center()
    .always_on_top(true)
    .focused(true)
    .build()
    .unwrap();

    'wait_login: loop {
        if login_window
            .url()
            .to_string()
            .starts_with("https://id.ouc.edu.cn/api/uia/index")
        {
            break 'wait_login;
        }
    }

    let (done_tx, done_rx) = oneshot::channel::<Vec<utils::Cookie>>();

    login_window
        .with_webview(move |webview: tauri::window::PlatformWebview| {
            #[cfg(target_os = "linux")]
            {
                use webkit2gtk::gio;
                use webkit2gtk::traits::{CookieManagerExt, WebContextExt, WebViewExt};

                let linux_webview = webview.inner();
                linux_webview.set_zoom_level(0.75);
                let webcontext = linux_webview.context().unwrap();
                let cookie_manager = webcontext.cookie_manager().unwrap();

                cookie_manager.cookies(
                    "https://id.ouc.edu.cn/sso",
                    None::<&gio::Cancellable>,
                    move |result: Result<Vec<soup::Cookie>, webkit2gtk::Error>| match result {
                        Ok(cookies) => {
                            println!("{}", cookies.len());
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
                    },
                );
            }

            #[cfg(windows)]
            unsafe {
                use webview2_com::Microsoft::Web::WebView2::Win32::{
                    ICoreWebView2, ICoreWebView2Controller, ICoreWebView2Cookie,
                    ICoreWebView2CookieManager, ICoreWebView2_2,
                };
                use webview2_com::{take_pwstr, GetCookiesCompletedHandler};
                use windows::core::Interface;
                use windows::core::HSTRING;
                use windows::core::PWSTR;

                let webview2_controller: ICoreWebView2Controller = webview.controller();
                webview2_controller.SetZoomFactor(0.75).unwrap();
                // get ICoreWebView2
                let core_webview2: ICoreWebView2 = webview2_controller.CoreWebView2().unwrap();
                let core_webview2_2: ICoreWebView2_2 = core_webview2.cast().unwrap();
                let cookie_manager: ICoreWebView2CookieManager =
                    core_webview2_2.CookieManager().unwrap();
                let uri = HSTRING::from("https://id.ouc.edu.cn/sso");

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
                                let mut cookie_str = vec![];
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
                                    cookie_str.push(utils::Cookie {
                                        name: take_pwstr(name),
                                        value: take_pwstr(value),
                                        domain: take_pwstr(domain),
                                        path: take_pwstr(path),
                                    });
                                }
                                done_tx.send(cookie_str).unwrap();
                            }
                            None => {
                                eprintln!("Error retrieving cookies");
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

    login_window.close().unwrap();

    let cookies: Vec<utils::Cookie> = done_rx.await.unwrap();

    for cookie in &cookies {
        println!("{:?}", cookie);
    }

    Ok(cookies)
}

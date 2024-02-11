use crate::utils;

use reqwest::header;

#[tauri::command]
pub async fn login_id_ouc_edu_cn(handle: tauri::AppHandle) -> Result<Vec<utils::Cookie>, String> {
    let login_window: tauri::Window = tauri::WindowBuilder::new(
        &handle,
        "login",
        tauri::WindowUrl::External("https://id.ouc.edu.cn/sso/login?service=https%3A%2F%2Fotrust.ouc.edu.cn%3A443%2Fpassport%2Fv1%2Fauth%2Fcas#/".parse().unwrap()),
    )
    .title("信息门户VPN登录")
    .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36 Edg/120.0.0.0 ITStudio")
    .inner_size(400., 800.)
    .center()
    .always_on_top(true)
    .focused(true)
    .build()
    .unwrap();

    'wait_login_vpn: loop {
        if login_window
            .url()
            .to_string()
            .starts_with("https://otrust.ouc.edu.cn/portal/service_center.html")
        {
            // 跳转到下一登录页面，URL为https://my-ouc-edu-cn-s.otrust.ouc.edu.cn
            login_window
                .eval("location.href = 'https://id-ouc-edu-cn-s.otrust.ouc.edu.cn/sso/login?service=https://my.ouc.edu.cn/cas/login#/'")
                .unwrap();
            break 'wait_login_vpn;
        }
    }
    'wait_login: loop {
        if login_window.url().to_string() == "https://my-ouc-edu-cn-s.otrust.ouc.edu.cn/#/home" {
            login_window.eval("location.href= 'https://id-ouc-edu-cn-s.otrust.ouc.edu.cn/sso/bridgeLogin?username=20020007043&service=https://zm.ouc.edu.cn/'")
            .unwrap();
            break 'wait_login;
        }
    }
    'wait_zm: loop {
        if login_window.url().to_string() == "https://zm-ouc-edu-cn-s.otrust.ouc.edu.cn/" {
            break 'wait_zm;
        }
    }

    let (done_tx, done_rx) = oneshot::channel::<Vec<utils::Cookie>>();

    login_window
        .with_webview(move |webview: tauri::window::PlatformWebview| {
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

    Ok(cookies)
}

#[tauri::command]
pub async fn get_score_pdf_url(
    cookies: Vec<utils::Cookie>,
    stu_id: String,
) -> Result<String, String> {
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    let mut req_cookies = String::new();
    for cookie in &cookies {
        if cookie.domain.eq(".otrust.ouc.edu.cn")
            || cookie.domain.eq("zm-ouc-edu-cn-s.otrust.ouc.edu.cn")
        {
            req_cookies.push_str(&format!("{}={}; ", cookie.name, cookie.value));
        }
    }
    let mut headers = header::HeaderMap::new();
    headers.insert(header::COOKIE, req_cookies.parse().unwrap());
    headers.insert("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/121.0.0.0 Safari/537.36 Edg/121.0.0.0".parse().unwrap());
    headers.insert("accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7".parse().unwrap());

    let res = client.post("https://zm-ouc-edu-cn-s.otrust.ouc.edu.cn/PrintService/GetElectronicEdtion?sf_request_type=ajax")
        .headers(headers.clone())
        .body(format!("{{\"TerminalNO\":\"\",\"TerminalIP\":\"\",\"TerminalType\":\"\",\"Data\":{{\"UserCode\":\"{}\",\"TemplateNO\":\"89\",\"id\":\"\",\"name\":\"\",\"code\":\"\"}}}}",stu_id))
        .send().await.unwrap();
    // 使用JSON解析器解析返回的JSON字符串，并生成Map
    let res_json: serde_json::Value = res.json().await.unwrap();
    let pdf_url = res_json["Data"]["TmpPDFUrl"].as_str().unwrap();

    Ok(format!(
        "https://zm-ouc-edu-cn-s.otrust.ouc.edu.cn/PrintService/DownloadFilePlat?filePath={}",
        pdf_url.split("filePath=").collect::<Vec<&str>>()[1]
    ))
}

#[tauri::command]
pub async fn get_pdf_blob(cookies: Vec<utils::Cookie>, pdf_url: String) -> Result<Vec<u8>, String> {
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    let mut req_cookies = String::new();
    for cookie in &cookies {
        if cookie.domain.eq(".otrust.ouc.edu.cn")
            || cookie.domain.eq("zm-ouc-edu-cn-s.otrust.ouc.edu.cn")
        {
            req_cookies.push_str(&format!("{}={}; ", cookie.name, cookie.value));
        }
    }
    let mut headers = header::HeaderMap::new();
    headers.insert(header::COOKIE, req_cookies.parse().unwrap());

    let res = client
        .get(pdf_url)
        .headers(headers.clone())
        .send()
        .await
        .unwrap();

    let pdf_blob = res.bytes().await.unwrap();
    Ok(pdf_blob.to_vec())
}

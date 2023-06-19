use wry::{
    application::{
        clipboard::Clipboard,
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    },
    webview::{WebView, WebViewBuilder},
};

use url::Url;

fn main() {
    let url: String = read_clipboard();
    let _page = render(&url);
}

fn read_clipboard() -> String {
    #[cfg(target_os = "linux")]
    gtk::init().unwrap();

    let cliboard = Clipboard::new();

    let content = cliboard.read_text().unwrap();

    let webview_url = generate_url(content);

    return webview_url;
}

fn render(url: &str) -> wry::Result<()> {
    enum UserEvent {
        Navigation(String),
    }

    // event loop
    let event_loop: EventLoop<UserEvent> = EventLoop::with_user_event();
    let proxy = event_loop.create_proxy();

    // window logic
    let window = WindowBuilder::new()
        .with_title("Medium Reader")
        .build(&event_loop)?;

    // webview logic
    let webview = WebViewBuilder::new(window)?
        .with_url(url)?
        .with_document_title_changed_handler(move |_window, title| {
            let _ = proxy.send_event(UserEvent::Navigation(title));
        })
        .build()?;

    // event loop
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => println!("Wry has started!"),
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                let _cleared_webview = *control_flow = ControlFlow::Exit;
            }
            Event::UserEvent(UserEvent::Navigation(_title)) => {
                let _ = WebView::clear_all_browsing_data(&webview);
            }
            _ => (),
        }
    });
}

fn generate_url(content: String) -> String {
    let base_url = "https://medium.com";

    let url = Url::parse(&base_url).unwrap();

    let url_parse_data = Url::parse(&content).unwrap_or(url);

    let mut parsed_url = url_parse_data.host_str().unwrap_or(base_url);

    let medium_custom_domains: Vec<&str> = vec![
        "medium.com",
        "blog.discord.com",
        "uxdesign.cc",
        "towardsdatascience.com",
        "hackernoon.com",
        "medium.freecodecamp.org",
        "betterhumans.coach.me",
        "codeburst.io",
        "medium.mybridge.co",
        "uxdesign.cc",
        "levelup.gitconnected.com",
        "itnext.io",
        "entrepreneurshandbook.co",
        "proandroiddev.com",
        "blog.prototypr.io",
        "thebolditalic.com",
        "blog.usejournal.com",
    ];

    if !medium_custom_domains.contains(&parsed_url) {
        parsed_url = "https://medium.com"
    } else {
        parsed_url = url_parse_data.as_str()
    }

    return parsed_url.to_string();
}

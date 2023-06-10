use wry::{
    application::{
        clipboard::Clipboard,
        // menu::{ContextMenu, MenuItemAttributes},
        // system_tray::{Icon, SystemTrayBuilder},
        // TrayId,
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    },
    webview::WebViewBuilder,
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
    // event loop
    let event_loop: EventLoop<()> = EventLoop::new();

    // window logic
    let window = WindowBuilder::new()
        .with_title("Medium Reader")
        .build(&event_loop)?;

    // webview logic
    let _webview = WebViewBuilder::new(window)?.with_url(url)?.build()?;

    // event loop
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => println!("Wry has started!"),
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}

fn generate_url(content: String) -> String {
    let url = Url::parse("https://medium.com").unwrap();

    let url_parse_data = Url::parse(&content).unwrap_or(url);

    let mut parsed_url = url_parse_data.host_str().unwrap();

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

// fn tray_init(&event_loop) {
//     let mut tray_menu = ContextMenu::new();

//     tray_menu.add_item(MenuItemAttributes::new("Quit"));

//     let path = concat!(env!("CARGO_MANIFEST_DIR"), "/src/assets/logo.png");

//     let icon = load_icon(std::path::Path::new(path));

//     let _system_tray = SystemTrayBuilder::new(icon.clone(), Some(tray_menu))
//         .with_id(TrayId::new("main-tray"))
//         .build(&event_loop)
//         .unwrap();
// }

// fn load_icon(path: &std::path::Path) -> Icon {
//     let (icon_rgba, icon_width, icon_height) = {
//         let image = image::open(path)
//             .expect("Failed to open icon path")
//             .into_rgba8();
//         let (width, height) = image.dimensions();
//         let rgba = image.into_raw();
//         (rgba, width, height)
//     };
//     Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
// }

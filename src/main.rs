use wry::{
    application::{
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        // menu::{ContextMenu, MenuItemAttributes},
        // system_tray::{Icon, SystemTrayBuilder},
        window::WindowBuilder,
        // TrayId,
    },
    webview::WebViewBuilder,
};

use cli_clipboard;

fn main() {
    let url: String = read_clipboard();
    let _page = render(&url);
}

fn read_clipboard() -> String {
    let mut clipboard_data = cli_clipboard::get_contents().unwrap();
    if clipboard_data.is_empty() {
        // if clip
        clipboard_data = "https://medium.com/".to_string()
    }
    return clipboard_data;
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

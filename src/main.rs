use wry::{
    application::{
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        menu::{ContextMenu, MenuItemAttributes},
        system_tray::{Icon, SystemTrayBuilder},
        window::WindowBuilder,
    },
    webview::WebViewBuilder,
};

fn load_icon(path: &std::path::Path) -> Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}

// fn add_tray(event_loop) {

// }

fn render(url: &str) -> wry::Result<()> {
    // event loop
    let event_loop: EventLoop<()> = EventLoop::new();

    // add_tray(&event_loop);

    let mut tray_menu = ContextMenu::new();

    tray_menu.add_item(MenuItemAttributes::new("Quit"));

    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/src/assets/logo.png");

    // let path = "src/assets/logo.png";

    let icon = load_icon(std::path::Path::new(path));

    let _system_tray = SystemTrayBuilder::new(icon.clone(), Some(tray_menu))
        .build(&event_loop)
        .unwrap();

    // window logic
    let window = WindowBuilder::new()
        .with_title("Medium Reader")
        .build(&event_loop)?;
    let _webview = WebViewBuilder::new(window)?.with_url(url)?.build()?;

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

fn main() {
    let _page = render("https://medium.com/spatial-data-science/how-to-extract-locations-from-text-with-natural-language-processing-9b77035b3ea4");
}

#![allow(non_snake_case)]
use dioxus::prelude::*;

fn main() {
    LaunchBuilder::new()
        .with_cfg(desktop!({
            use dioxus::desktop::{Config, LogicalSize, WindowBuilder};
            Config::new().with_window(
                WindowBuilder::default()
                    .with_title("krpc rust Dashboard")
                    .with_inner_size(LogicalSize::new(1280.0, 1024.0)),
            )
        }))
        .launch(App);
}

fn App() -> Element {
    None
}

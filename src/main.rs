extern crate enigo;

use std::time::Duration;
use eframe::{NativeOptions, run_native};
use enigo::*;
use crate::adapter::click_behavior::KeyBoardClick;
use crate::model::enigo_devices::keyboard::KeyBoard;
use crate::views::index::MyEguiApp;

mod service;
mod model;
mod adapter;
mod common;
mod views;

fn main() -> eframe::Result<()> {
    //let mut keyBoard = KeyBoard::new();
    //keyBoard.click("ssss");
    //keyBoard.click(&Key::Tab)
    //keyBoard.click_hold("sa", Duration::from_secs(50))
    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    run_native(
        "My egui App",
        options,
        Box::new(|cc| {
            // This gives us image support:
            //egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::<MyEguiApp>::default()
        }),
    )
}
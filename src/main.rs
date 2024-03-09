extern crate enigo;

use std::time::{Duration, SystemTime};
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

use msdk_driver_adapter::{MsdkAdapter, MsdkKeyBoardOperation};


fn main() -> Result<(), eframe::Error>  {
//let mut keyBoard = KeyBoard::new();
    //keyBoard.click("ssss");
    //keyBoard.click(&Key::Tab)
    //keyBoard.click_hold("sa", Duration::from_secs(50))
   /* let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1000.0, 1000.0]),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {
            // This gives us image support:
            //egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::<MyEguiApp>::default()
        }),a
    )*/

    let msdk = MsdkAdapter::new(1).unwrap();
    msdk.key_press(65, 1).expect("TODO: panic message");
    let res = msdk.close().expect("TODO: panic message");
    println!("{:?}", res);
    Ok(())
}
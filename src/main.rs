#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(non_snake_case)]

use fltk::{
    app,
    button::Button,
    enums::Align,
    enums::Color,
    enums::Font,
    frame::Frame,
    group::{Column, Flex},
    image::PngImage,
    prelude::*,
    window::Window,
};
use fltk_theme::{color_themes, widget_themes, ColorTheme};
use std::ffi::c_void;
use windows_sys::Win32::Foundation::HWND;
use windows_sys::Win32::Graphics::Dwm::{
    DwmSetWindowAttribute, DWMWA_USE_IMMERSIVE_DARK_MODE,
};

use lcu::GameClient;

use std::sync::{Arc, Mutex};
use std::thread;

mod lcu;
mod models;

enum ChannelMsg {
    Text(String),
    ChromaColor(u32),
    ClientStatus(bool),
}

fn main() {
    let icon_app_bytes = include_bytes!("assets/icon.png");
    let icon_btn_skin_bytes = include_bytes!("assets/icon_skin.png");
    let icon_btn_chroma_bytes = include_bytes!("assets/icon_chroma.png");
    let icon_status_grey_bytes = include_bytes!("assets/icon_status_grey.png");
    let icon_status_green_bytes =
        include_bytes!("assets/icon_status_green.png");
    let icon_status_red_bytes = include_bytes!("assets/icon_status_red.png");
    let icon_app = PngImage::from_data(icon_app_bytes).unwrap();
    let icon_skin = PngImage::from_data(icon_btn_skin_bytes).unwrap();
    let icon_chroma = PngImage::from_data(icon_btn_chroma_bytes).unwrap();
    let icon_status_grey = PngImage::from_data(icon_status_grey_bytes).unwrap();
    let icon_status_green =
        PngImage::from_data(icon_status_green_bytes).unwrap();
    let icon_status_red = PngImage::from_data(icon_status_red_bytes).unwrap();

    // Inizialize lcu client and channel for updating the gui
    let (s, r) = app::channel::<ChannelMsg>();
    let client = Arc::new(Mutex::new(GameClient::new()));
    let (c1, c2, c3) = (client.clone(), client.clone(), client.clone());

    let app = app::App::default();
    let theme = ColorTheme::new(color_themes::BLACK_THEME);
    theme.apply();

    let mut win = Window::default()
        .with_size(260, 125)
        .with_label("Skin Randomizer");

    win.set_icon(Some(icon_app));

    let mut column = Column::default_fill();
    column.set_spacing(5);
    column.set_margin(10);

    let mut text = Frame::default()
        .with_label(&format!("U{}U", char::from_u32(0x03C9).unwrap()));
    text.set_label_font(Font::Helvetica);
    text.set_label_size(18);
    text.set_align(Align::TextNextToImage);

    let mut group_btns = Flex::default_fill();
    group_btns.set_margins(0, 0, 0, 5);

    let mut btn_skin = Button::default().with_label("  Skin");
    btn_skin.set_label_font(Font::Helvetica);
    btn_skin.set_label_size(16);
    btn_skin.set_color(Color::Dark2);
    btn_skin.set_image(Some(icon_skin));
    btn_skin.set_align(Align::ImageNextToText);
    btn_skin.set_frame(widget_themes::OS_BUTTON_UP_BOX);
    btn_skin.set_callback(move |_| {
        let c1 = c1.clone();
        thread::spawn(move || match c1.lock() {
            Ok(mut g) => match g.set_skin() {
                Ok(skin_name) => {
                    s.send(ChannelMsg::Text(skin_name));
                }
                Err(e) => {
                    s.send(ChannelMsg::Text(e));
                }
            },
            Err(e) => {
                dbg!(e);
            }
        });
    });

    let mut btn_chroma = Button::default().with_label("  Chroma");
    btn_chroma.set_label_font(Font::Helvetica);
    btn_chroma.set_label_size(16);
    btn_chroma.set_color(Color::Dark2);
    btn_chroma.set_image(Some(icon_chroma));
    btn_chroma.set_align(Align::ImageNextToText);
    btn_chroma.set_frame(widget_themes::OS_BUTTON_UP_BOX);
    btn_chroma.set_callback(move |_| {
        let c2 = c2.clone();
        thread::spawn(move || match c2.lock() {
            Ok(g) => match g.set_chroma() {
                Ok(chroma) => {
                    s.send(ChannelMsg::Text(chroma.0));
                    s.send(ChannelMsg::ChromaColor(chroma.1));
                }
                Err(e) => {
                    s.send(ChannelMsg::Text(e));
                }
            },
            Err(e) => {
                dbg!(e);
            }
        });
    });

    group_btns.end();

    let mut statusbar = Frame::default().with_label(" Client");
    statusbar.set_label_font(Font::HelveticaItalic);
    statusbar.set_align(Align::Inside | Align::Left | Align::ImageNextToText);
    statusbar.set_label_size(12);
    statusbar.set_image(Some(icon_status_grey));

    column.fixed(&text, 40);
    column.fixed(&group_btns, 40);
    column.fixed(&statusbar, 15);
    column.end();

    win.end();
    win.show();

    // Using the win32 api to make the window title bar dark, since fltk doesnt support it
    // Get the window handle (HWND)
    let hwnd = win.raw_handle() as HWND;
    if hwnd != 0 {
        let use_dark_mode: i32 = 1;
        unsafe {
            // Set the window attribute to use dark mode
            DwmSetWindowAttribute(
                hwnd,
                DWMWA_USE_IMMERSIVE_DARK_MODE as u32,
                &use_dark_mode as *const i32 as *const c_void,
                std::mem::size_of::<i32>() as u32,
            );
        }
    }

    // Background thread to continuously check if the league client is running
    let check_client_status = move |handle| {
        let c3 = c3.clone();
        thread::spawn(move || {
            if let Ok(mut c3) = c3.lock() {
                match c3.status() {
                    true => s.send(ChannelMsg::ClientStatus(true)),
                    false => match c3.retry() {
                        Ok(_) => s.send(ChannelMsg::ClientStatus(true)),
                        Err(e) => {
                            dbg!(e);
                            s.send(ChannelMsg::ClientStatus(false))
                        }
                    },
                }
            }
        });
        app::repeat_timeout3(1.0, handle);
    };
    app::add_timeout3(1.0, check_client_status);

    while app.wait() {
        if let Some(v) = r.recv() {
            match v {
                ChannelMsg::Text(t) => {
                    text.set_label_color(Color::ForeGround);
                    text.set_label(&t);
                }
                ChannelMsg::ChromaColor(c) => {
                    text.set_label_color(Color::from_hex(c));
                }
                ChannelMsg::ClientStatus(status) => {
                    if status {
                        statusbar.set_image(Some(icon_status_green.clone()));
                    } else {
                        statusbar.set_image(Some(icon_status_red.clone()));
                    }
                }
            }
            win.redraw();
        }
    }
}

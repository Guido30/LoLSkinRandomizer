#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(non_snake_case)]

use fltk::{
    app,
    button::Button,
    enums::Color,
    enums::Font,
    enums::{Align, Event},
    frame::Frame,
    group::{Column, Flex},
    image::PngImage,
    prelude::*,
    window::Window,
};
use fltk_evented::Listener;
use fltk_theme::{
    color_themes, widget_themes, ColorTheme, SchemeType, WidgetScheme,
};
use rand::{seq::SliceRandom, thread_rng};

use lcu::GameClient;

mod lcu;
mod models;

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
    let mut client = GameClient::new();

    let app = app::App::default();
    let theme = ColorTheme::new(color_themes::BLACK_THEME);
    theme.apply();
    let widget_scheme = WidgetScheme::new(SchemeType::Clean);
    widget_scheme.apply();

    let mut win = Window::default()
        .with_size(260, 125)
        .with_label("Skin Randomizer");

    win.set_icon(Some(icon_app));

    let wrapper: Listener<_> = Frame::default().with_size(260, 125).into();

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

    let mut btn_skin: Listener<_> =
        Button::default().with_label("  Skin").into();
    btn_skin.set_label_font(Font::Helvetica);
    btn_skin.set_label_size(16);
    btn_skin.set_color(Color::Dark2);
    btn_skin.set_image(Some(icon_skin));
    btn_skin.set_align(Align::ImageNextToText);
    btn_skin.set_frame(widget_themes::OS_BUTTON_UP_BOX);

    let mut btn_chroma: Listener<_> =
        Button::default().with_label("  Chroma").into();
    btn_chroma.set_label_font(Font::Helvetica);
    btn_chroma.set_label_size(16);
    btn_chroma.set_color(Color::Dark2);
    btn_chroma.set_image(Some(icon_chroma));
    btn_chroma.set_align(Align::ImageNextToText);
    btn_chroma.set_frame(widget_themes::OS_BUTTON_UP_BOX);

    group_btns.end();

    let mut statusbar = Frame::default().with_label(" Client");
    statusbar.set_label_font(Font::HelveticaItalic);
    statusbar.set_align(Align::Inside | Align::Left | Align::ImageNextToText);
    statusbar.set_label_size(12);
    statusbar.set_image(Some(icon_status_grey));

    column.fixed(&mut text, 40);
    column.fixed(&mut group_btns, 40);
    column.fixed(&mut statusbar, 15);
    column.end();

    win.end();
    win.show();

    while app.wait() {
        match wrapper.event() {
            Event::Enter | Event::Leave => {
                if !client.status() {
                    client.retry();
                }
                if client.status() {
                    statusbar.set_image(Some(icon_status_green.clone()));
                    win.redraw();
                } else {
                    statusbar.set_image(Some(icon_status_red.clone()));
                    win.redraw();
                }
            }
            _ => (),
        }

        if btn_skin.triggered() {
            text.set_label_color(Color::ForeGround);
            if client.status() {
                let summoner_id = client
                    .call_summoner_v1_current_summoner_account_and_summoner_ids(
                    );
                let skin_ids = client
                    .call_champ_select_v1_pickable_skin_ids()
                    .map_err(|_| text.set_label("Not in champion select!"));
                let current_champ =
                    client.call_champ_select_v1_current_champion();
                let mut rng = thread_rng();

                #[allow(unused_must_use)]
                {
                    summoner_id.and_then(|summoner_id| {
                        skin_ids.and_then(|skin_ids| {
                            current_champ.and_then(|current_champ| {
                                client.call_champions_v1_inventories_summonerid_champions_championid_skins(
                                    summoner_id.summoner_id,
                                    current_champ as i64,
                                ).map_err(|_| {text.set_label("Champion not picked yet!")}).and_then(|champ_skin_ids| {
                                    let champ_skin_ids: Vec<(i32, String)> = champ_skin_ids.iter()
                                        .filter(|skin| skin_ids.contains(&skin.id))
                                        .map(|skin| (skin.id, skin.name.clone()))
                                        .collect();
                                    let random_skin = champ_skin_ids.choose(&mut rng);

                                    if let Some(skin) = random_skin {
                                        client.call_champ_select_v1_session_my_selection(skin.0).and_then(|_| {
                                            text.set_label(&skin.1);
                                            Ok(())
                                        });
                                    };
                                    Ok(())
                                });
                                Ok(())
                            });
                            Ok(())
                        });
                        Ok(())
                    });
                }
            } else {
                text.set_label("LeagueClient not found!");
            }
        }

        if btn_chroma.triggered() {
            text.set_label_color(Color::ForeGround);
            if client.status() {
                let summoner_id = client
                    .call_summoner_v1_current_summoner_account_and_summoner_ids(
                    );
                let champ_select = client
                    .call_champ_select_v1_session()
                    .map_err(|_| text.set_label("Not in champion select!"));
                let mut rng = thread_rng();

                #[allow(unused_must_use)]
                {
                    summoner_id.and_then(|summoner_id| {
                        champ_select.and_then(|champ_select| {
                            let summoner =
                                champ_select.my_team.iter().find(|summoner| {
                                    summoner.summoner_id == summoner_id.summoner_id
                                });
                            let selected_skin_id = match summoner {
                                Some(s) => s.selected_skin_id,
                                None => 0,
                            };
                            let champion_id = match summoner {
                                Some(s) => s.champion_id,
                                None => 0,
                            };
                            if champion_id != 0 {
                                client.call_champions_v1_inventories_summonerid_champions_championid_skins(summoner_id.summoner_id, champion_id as i64).and_then(|skin_collection| {
                                    let mut current_chromas = Vec::new();
                                    let mut random_chroma = None;
                                    let current_skin = skin_collection.iter().find(|skin| skin.id == selected_skin_id);
                                    for chroma in skin_collection.clone().into_iter().map(|skin| skin.chromas) {
                                        if chroma.iter().any(|chr| chr.id == selected_skin_id) {
                                            current_chromas = chroma.iter().filter(|chr| chr.ownership.owned).map(|chr| (chr.id, chr.colors[0].clone())).collect();
                                        }
                                    }
                                    if current_chromas.is_empty() {
                                        if let Some(current_skin) = current_skin {
                                            current_chromas = current_skin.chromas.iter().filter(|chr| chr.ownership.owned).map(|chr| (chr.id, chr.colors[0].clone())).collect();
                                            random_chroma = current_chromas.choose(&mut rng);
                                        }
                                    } else {
                                        random_chroma = current_chromas.choose(&mut rng);
                                    }
                                    if let Some(chroma) = random_chroma {
                                        client.call_champ_select_v1_session_my_selection(chroma.0).and_then(|_| {
                                            text.set_label("Randomized Chroma");
                                            if let Ok(color) = u32::from_str_radix(&chroma.1[1..], 16) {
                                                text.set_label_color(Color::from_hex(color));
                                            }
                                            win.redraw();
                                            Ok(())
                                        });
                                    };
                                    Ok(())
                                });
                            } else {
                                text.set_label("Champion not picked yet!");
                            }
                            Ok(())
                        });
                        Ok(())
                    });
                }
            } else {
                text.set_label("LeagueClient not found!");
            }
        }
    }
}

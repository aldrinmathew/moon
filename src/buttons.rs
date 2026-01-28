use gtk4::{
    ApplicationWindow, Button, Image,
    gdk::{Cursor, Texture},
    glib::Bytes,
    prelude::{ButtonExt, GtkWindowExt},
};

pub fn close_button(window: ApplicationWindow) -> Button {
    let button = Button::builder()
        .hexpand(false)
        .css_name("close_button")
        .margin_start(5)
        .width_request(25)
        .height_request(25)
        .child(
            &Image::builder()
                .paintable(
                    &Texture::from_bytes(&Bytes::from(include_bytes!("./images/star.png")))
                        .unwrap(),
                )
                .pixel_size(25)
                .opacity(0.5)
                .build(),
        )
        .cursor(&Cursor::from_name("pointer", None).unwrap())
        .build();
    button.connect_clicked(move |_| {
        window.close();
    });
    return button;
}

pub fn maximize_button(window: ApplicationWindow) -> Button {
    let maximize_button = Button::builder()
        .name("maximize_button")
        .hexpand(false)
        .css_name("maximize_button")
        .margin_start(7)
        .width_request(25)
        .height_request(25)
        .child(
            &Image::builder()
                .paintable(
                    &Texture::from_bytes(&Bytes::from(include_bytes!("./images/plus.png")))
                        .unwrap(),
                )
                .pixel_size(25)
                .opacity(0.5)
                .build(),
        )
        .cursor(&Cursor::from_name("pointer", None).unwrap())
        .build();
    maximize_button.connect_clicked(move |_| {
        window.set_maximized(!window.is_maximized());
    });
    return maximize_button;
}

pub fn minimize_button(window: ApplicationWindow) -> Button {
    let minimize_button = Button::builder()
        .name("minimize_button")
        .hexpand(false)
        .css_name("minimize_button")
        .width_request(25)
        .width_request(25)
        .child(
            &Image::builder()
                .paintable(
                    &Texture::from_bytes(&Bytes::from(include_bytes!("./images/minus.png")))
                        .unwrap(),
                )
                .pixel_size(25)
                .opacity(0.5)
                .build(),
        )
        .cursor(&Cursor::from_name("pointer", None).unwrap())
        .build();
    minimize_button.connect_clicked(move |_| {
        window.minimize();
    });
    return minimize_button;
}

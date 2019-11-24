use gtk::prelude::*;
use gtk::Inhibit;
use gtk::Orientation::Vertical;
use relm::{init, Component, Widget};
use relm_derive::{widget, Msg};

use self::HeaderMsg::*;
use self::WinMsg::*;

#[derive(Msg)]
pub enum HeaderMsg {
    Add,
    Remove,
}

#[widget]
impl Widget for Header {
    fn model() -> () {}

    fn update(&mut self, event: HeaderMsg) {
        match event {
            Add => println!("Add"),
            Remove => println!("Remove"),
        }
    }

    view! {
        #[name="titlebar"]
        gtk::HeaderBar {
            title: Some("title"),
            show_close_button: true,

            #[name="add_button"]
            gtk::Button {
                clicked => Add,
                label: "Add",
            },

            #[name="remove_button"]
            gtk::Button {
                clicked => Remove,
                label: "Remove",
            },
        }
    }
}

pub struct Model {
    header: Component<Header>,
}

#[derive(Msg)]
pub enum WinMsg {
    Quit,
}

#[widget]
impl Widget for Win {
    fn model() -> Model {
        let header = init::<Header>(()).expect("Header");

        Model { header }
    }

    fn update(&mut self, event: WinMsg) {
        match event {
            Quit => gtk::main_quit(),
        }
    }

    view! {
        gtk::Window {
            titlebar: Some(self.model.header.widget()),

            #[name = "app"]
            gtk::Box {
                orientation: Vertical
            },

            delete_event(_, _) => (Quit, Inhibit(false)),
        }
    }
}

fn main() {
    Win::run(()).expect("Window::run");
}

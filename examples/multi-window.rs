use gtk::{Inhibit, WidgetExt};
use relm::{init, Component, Widget};
use relm_derive::{widget, Msg};

use self::Msg::*;

#[widget]
impl Widget for SecondaryWin {
    fn model() -> () {}

    fn update(&mut self, _msg: Msg) {}

    view! {
        gtk::Window {
            delete_event(_, _) => (Quit, Inhibit(false)),
        }
    }
}

pub struct Model {
    _win: Component<SecondaryWin>,
}

#[derive(Msg)]
pub enum Msg {
    Quit,
}

#[widget]
impl Widget for Win {
    fn model() -> Model {
        Model {
            _win: init::<SecondaryWin>(()).expect("secondary window"),
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Quit => gtk::main_quit(),
        }
    }

    view! {
        gtk::Window {
            delete_event(_, _) => (Quit, Inhibit(false)),
        }
    }
}

fn main() {
    Win::run(()).unwrap();
}

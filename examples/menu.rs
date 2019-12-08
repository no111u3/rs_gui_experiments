use gtk::Orientation::Vertical;
use gtk::{GtkMenuItemExt, Inhibit, MenuShellExt, OrientableExt, WidgetExt};
use relm::{connect, Relm, Widget};
use relm_derive::{widget, Msg};

use self::Msg::*;

pub struct Model {
    relm: Relm<Win>,
}

#[derive(Msg)]
pub enum Msg {
    Quit,
}

#[widget]
impl Widget for Win {
    fn init_view(&mut self) {
        let file_menu = gtk::Menu::new();
        let file_item = gtk::MenuItem::new_with_label("File");
        file_item.set_submenu(Some(&file_menu));
        let quit_item = gtk::MenuItem::new_with_label("Quit");
        self.menubar.append(&file_item);
        file_menu.append(&quit_item);
        self.menubar.show_all();

        connect!(quit_item, connect_activate(_), self.model.relm, Quit);
    }

    fn model(relm: &Relm<Self>, _: ()) -> Model {
        Model { relm: relm.clone() }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Quit => gtk::main_quit(),
        }
    }

    view! {
        gtk::Window {
            gtk::Box {
                orientation: Vertical,
                #[name="menubar"]
                gtk::MenuBar {
                },
            },
            delete_event(_, _) => (Quit, Inhibit(false)),
        }
    }
}

fn main() {
    Win::run(()).expect("Win::run failed");
}

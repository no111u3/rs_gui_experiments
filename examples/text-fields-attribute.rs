use gtk::Orientation::Vertical;
use gtk::{EditableSignals, EntryExt, Inhibit, LabelExt, OrientableExt, WidgetExt};
use relm::Widget;
use relm_derive::{widget, Msg};

use self::Msg::*;

pub struct Model {
    content: String,
}

#[derive(Msg)]
pub enum Msg {
    Change(String, usize),
    Quit,
}

#[widget]
impl Widget for Win {
    fn model() -> Model {
        Model {
            content: String::new(),
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Change(text, len) => {
                self.model.content = text.chars().rev().collect();
                self.model.content += &format!(" ({})", len);
            }
            Quit => gtk::main_quit(),
        }
    }

    view! {
        gtk::Window {
            gtk::Box {
                orientation: Vertical,
                #[name="entry"]
                gtk::Entry {
                    changed(entry) => {
                        let text = entry.get_text().expect("get_text failed").to_string();
                        let len = text.len();
                        Change(text, len)
                    },
                    placeholder_text: Some("Text to reverse"),
                },
                #[name="label"]
                gtk::Label {
                    text: &self.model.content,
                },
            },
            delete_event(_, _) => (Quit, Inhibit(false)),
        }
    }
}

fn main() {
    Win::run(()).expect("Win::run failed");
}

#[cfg(test)]
mod tests {
    use gdk::enums::key;
    use gtk::LabelExt;

    use gtk_test::{assert_text, enter_key, enter_keys};

    use crate::Win;

    #[test]
    fn label_change() {
        let (_component, widgets) = relm::init_test::<Win>(()).expect("init_test failed");
        let entry = &widgets.entry;
        let label = &widgets.label;

        assert_text!(label, "");

        enter_keys(entry, "test");
        assert_text!(label, "tset (4)");

        enter_key(entry, key::BackSpace);
        assert_text!(label, "set (3)");

        enter_key(entry, key::Home);
        //enter_key(entry, key::Delete); // TODO: when supported by enigo.
        enter_keys(entry, "a");
        assert_text!(label, "seta (4)");

        enter_key(entry, key::End);
        enter_keys(entry, "a");
        //assert_text!(label, "aseta (5)"); // FIXME
    }
}

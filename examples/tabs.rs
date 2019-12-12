use gtk::{ButtonExt, Inhibit, LabelExt, NotebookExt, WidgetExt};
use relm::Widget;
use relm_derive::{widget, Msg};

use self::Msg::*;

#[derive(Msg)]
pub enum Msg {
    Quit,
}

#[widget]
impl Widget for Win {
    fn model() -> () {
        ()
    }

    fn update(&mut self, event: Msg) {
        match event {
            Quit => gtk::main_quit(),
        }
    }

    view! {
        gtk::Window {
            #[name="tabs"]
            gtk::Notebook {
                #[name="inc_button"]
                gtk::Button {
                    child: {
                        tab_label: Some("First Button"),
                    },
                    label: "Button",
                },
                #[name="label"]
                gtk::Label {
                    tab: {
                        label: Some(&gtk::Label::new(Some("Second page"))),
                    },
                    text: "Hello",
                },
                #[name="dec_button"]
                gtk::Button {
                    label: "Another Button",
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
    use gtk::{Cast, Label, LabelExt, NotebookExt};
    use gtk_test::assert_text;

    use crate::Win;

    #[test]
    fn root_widget() {
        let (_component, widgets) = relm::init_test::<Win>(()).expect("init_test failed");
        let tabs = &widgets.tabs;
        let inc_button = &widgets.inc_button;
        let label = &widgets.label;
        let dec_button = &widgets.dec_button;

        assert_eq!(
            tabs.get_tab_label_text(inc_button)
                .expect("inc button label"),
            "First Button"
        );
        let label_widget: Label = tabs
            .get_tab_label(label)
            .expect("label widget")
            .downcast::<Label>()
            .expect("downcast");
        assert_text!(label_widget, "Second page");
        assert_eq!(tabs.get_tab_label(dec_button), None);
        assert_eq!(tabs.get_tab_label_text(dec_button), None);
    }
}

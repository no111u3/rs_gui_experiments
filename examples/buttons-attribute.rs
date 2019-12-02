use gtk::{ButtonExt, Inhibit, LabelExt, OrientableExt, WidgetExt};

use gtk::Orientation::Vertical;
use relm::Widget;
use relm_derive::{widget, Msg};

use self::Msg::*;

// Define the structure of the model.
pub struct Model {
    counter: i32,
}

// The messages that can be sent to the update function.
#[derive(Msg)]
pub enum Msg {
    #[cfg(test)]
    Test,
    Decrement,
    Increment,
    Quit,
}

#[widget]
impl Widget for Win {
    // The initial model.
    fn model() -> Model {
        Model { counter: 0 }
    }

    // Update the model according to the message received.
    fn update(&mut self, event: Msg) {
        match event {
            #[cfg(test)]
            Test => (),
            Decrement => self.model.counter -= 1,
            Increment => self.model.counter += 1,
            Quit => gtk::main_quit(),
        }
    }

    view! {
        gtk::Window {
            gtk::Box {
                // Set the orientation property of the Box.
                orientation: Vertical,
                // Create a Button inside the Box.
                #[name="inc_button"]
                gtk::Button {
                    // Send the message Increment when the button is clicked.
                    clicked => Increment,
                    // TODO: check if using two events of the same name work.
                    label: "+",
                },
                #[name="label"]
                gtk::Label {
                    // Bind the text property of the label to the counter attribute of the model.
                    text: &self.model.counter.to_string(),
                },
                #[name="dec_button"]
                gtk::Button {
                    clicked => Decrement,
                    label: "-",
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
    use gtk::{ButtonExt, LabelExt};

    use gtk_test::{assert_label, assert_text, click};

    use crate::Win;

    #[test]
    fn label_change() {
        let (_component, widgets) = relm::init_test::<Win>(()).expect("init_test failed");
        let inc_button = &widgets.inc_button;
        let dec_button = &widgets.dec_button;
        let label = &widgets.label;

        assert_label!(inc_button, "+");
        assert_label!(dec_button, "-");

        assert_text!(label, 0);
        click(inc_button);
        assert_text!(label, 1);
        click(inc_button);
        assert_text!(label, 2);
        click(inc_button);
        assert_text!(label, 3);
        click(inc_button);
        assert_text!(label, 4);

        click(dec_button);
        assert_text!(label, 3);
        click(dec_button);
        assert_text!(label, 2);
        click(dec_button);
        assert_text!(label, 1);
        click(dec_button);
        assert_text!(label, 0);
        click(dec_button);
        assert_text!(label, -1);
    }
}

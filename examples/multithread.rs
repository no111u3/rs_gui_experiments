use std::thread;
use std::time::Duration;

use gtk::Orientation::Vertical;
use gtk::{Inhibit, LabelExt, OrientableExt, WidgetExt};
use relm::{Channel, Relm, Widget};
use relm_derive::{widget, Msg};

use self::Msg::*;

pub struct Model {
    _channel: Channel<i32>,
    text: String,
}

#[derive(Clone, Msg)]
pub enum Msg {
    Quit,
    Value(i32),
}

#[widget]
impl Widget for Win {
    fn model(relm: &Relm<Self>, _: ()) -> Model {
        let stream = relm.stream().clone();
        // Create a channel to be able to send a message from another thread.
        let (channel, sender) = Channel::new(move |num| {
            // This closure is executed whenever a message is received from the sender.
            // We send a message to the current widget.
            stream.emit(Value(num));
        });
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(200));
            // Send a message from the other thread.
            // The value 42 will be received as the num parameter in the above closure.
            sender.send(42).expect("send message");
        });
        Model {
            _channel: channel,
            text: "Computing...".to_string(),
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Quit => gtk::main_quit(),
            Value(num) => self.model.text = num.to_string(),
        }
    }

    view! {
        gtk::Window {
            gtk::Box {
                orientation: Vertical,
                gtk::Label {
                    text: &self.model.text,
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
    use relm_test::{relm_observer_new, relm_observer_wait};

    use crate::Msg::Value;
    use crate::Win;

    #[test]
    fn channel() {
        let (component, _widgets) = relm::init_test::<Win>(()).expect("init_test failed");
        let observer = relm_observer_new!(component, Value(_));
        relm_observer_wait!(let Value(value) = observer);
        assert_eq!(value, 42);
    }
}

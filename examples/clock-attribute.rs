use chrono::{DateTime, Local};
use gtk::{Inhibit, LabelExt, WidgetExt};
use relm::{interval, Relm, Widget};
use relm_derive::{widget, Msg};

use self::Msg::*;

pub struct Model {
    time: DateTime<Local>,
}

#[derive(Msg)]
pub enum Msg {
    Quit,
    Tick,
}

#[widget]
impl Widget for Win {
    fn model() -> Model {
        Model { time: Local::now() }
    }

    fn subscriptions(&mut self, relm: &Relm<Self>) {
        interval(relm.stream(), 1000, || Tick);
    }

    fn update(&mut self, event: Msg) {
        match event {
            Tick => self.model.time = Local::now(),
            Quit => gtk::main_quit(),
        }
    }

    view! {
        gtk::Window {
            #[name="label"]
            gtk::Label {
                text: &self.model.time.format("%H:%M:%S").to_string(),
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
    use chrono::{Local, NaiveTime};
    use gtk::LabelExt;

    use gtk_test::wait;

    use crate::Win;

    #[test]
    fn label_change() {
        let (_component, widgets) = relm::init_test::<Win>(()).expect("init_test failed");
        let label = &widgets.label;

        fn time_close(time1: glib::GString, time2: String) -> bool {
            println!("{}", time1);
            println!("{}", time2);
            let date1 = NaiveTime::parse_from_str(&time1, "%H:%M:%S").expect("parse time1");
            let date2 = NaiveTime::parse_from_str(&time2, "%H:%M:%S").expect("parse time2");
            (date1.signed_duration_since(date2)).num_seconds() <= 1
        }

        let time = Local::now();
        assert!(time_close(
            label.get_text().expect("text"),
            time.format("%H:%M:%S").to_string()
        ));

        wait(2000);

        let time2 = Local::now();
        assert_ne!(time, time2);
        assert!(time_close(
            label.get_text().expect("text"),
            time2.format("%H:%M:%S").to_string()
        ));
    }
}

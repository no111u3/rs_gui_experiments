use chrono::Local;
use gtk::{ContainerExt, Inhibit, Label, LabelExt, WidgetExt, Window, WindowType};
use relm::{connect, interval, Relm, Update, Widget, WidgetTest};
use relm_derive::Msg;

use self::Msg::*;

#[derive(Msg)]
enum Msg {
    Quit,
    Tick,
}

#[derive(Clone)]
struct Win {
    label: Label,
    window: Window,
}

impl Update for Win {
    type Model = ();
    type ModelParam = ();
    type Msg = Msg;

    fn model(_: &Relm<Self>, _: ()) -> () {
        ()
    }

    fn subscriptions(&mut self, relm: &Relm<Self>) {
        interval(relm.stream(), 1000, || Tick);
    }

    fn update(&mut self, event: Msg) {
        match event {
            Tick => {
                let time = Local::now();
                self.label.set_text(&format!("{}", time.format("%H:%M:%S")));
            }
            Quit => gtk::main_quit(),
        }
    }
}

impl Widget for Win {
    type Root = Window;

    fn root(&self) -> Self::Root {
        self.window.clone()
    }

    fn view(relm: &Relm<Self>, _model: Self::Model) -> Self {
        let label = Label::new(None);

        let window = Window::new(WindowType::Toplevel);

        window.add(&label);

        window.show_all();

        connect!(
            relm,
            window,
            connect_delete_event(_, _),
            return (Some(Quit), Inhibit(false))
        );

        let mut win = Win {
            label: label,
            window: window,
        };

        win.update(Tick);
        win
    }
}

impl WidgetTest for Win {
    type Widgets = Win;

    fn get_widgets(&self) -> Self::Widgets {
        self.clone()
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

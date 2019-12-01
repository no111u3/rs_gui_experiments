use gtk::{
    Button, ButtonExt, ContainerExt, Inhibit, Label, LabelExt, WidgetExt, Window, WindowType,
};

use gtk::Orientation::Vertical;
use relm::{connect, Relm, Update, Widget, WidgetTest};
use relm_derive::Msg;

struct Model {
    counter: i32,
}

#[derive(Msg)]
enum Msg {
    Decrement,
    Increment,
    Quit,
}

#[derive(Clone)]
struct Widgets {
    counter_label: Label,
    minus_button: Button,
    plus_button: Button,
    window: Window,
}

struct Win {
    model: Model,
    widgets: Widgets,
}

impl Update for Win {
    // Specify the model used for this widget.
    type Model = Model;
    // Specify the model parameter used to init.
    type ModelParam = ();
    // Specify the type of the messages sent to the update function.
    type Msg = Msg;

    fn model(_: &Relm<Self>, _: ()) -> Model {
        Model { counter: 0 }
    }

    fn update(&mut self, event: Msg) {
        let label = &self.widgets.counter_label;

        match event {
            Msg::Decrement => {
                self.model.counter -= 1;
                // Manually update the view.
                label.set_text(&self.model.counter.to_string());
            }
            Msg::Increment => {
                self.model.counter += 1;
                label.set_text(&self.model.counter.to_string());
            }
            Msg::Quit => gtk::main_quit(),
        }
    }
}

impl Widget for Win {
    // Specify the type of the root widget.
    type Root = Window;

    // Returns the root widget.
    fn root(&self) -> Self::Root {
        self.widgets.window.clone()
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        // Create the view using the normal GTK+ method calls.
        let vbox = gtk::Box::new(Vertical, 0);

        let plus_button = Button::new_with_label("+");
        vbox.add(&plus_button);

        let counter_label = Label::new(Some("0"));
        vbox.add(&counter_label);

        let minus_button = Button::new_with_label("-");
        vbox.add(&minus_button);

        let window = Window::new(WindowType::Toplevel);

        window.add(&vbox);

        window.show_all();

        // Send the message Increment when the button is clicked.
        connect!(relm, plus_button, connect_clicked(_), Msg::Increment);
        connect!(relm, minus_button, connect_clicked(_), Msg::Decrement);
        connect!(
            relm,
            window,
            connect_delete_event(_, _),
            return (Some(Msg::Quit), Inhibit(false))
        );

        Win {
            model,
            widgets: Widgets {
                counter_label,
                minus_button,
                plus_button,
                window: window,
            },
        }
    }
}

impl WidgetTest for Win {
    type Widgets = Widgets;

    fn get_widgets(&self) -> Self::Widgets {
        self.widgets.clone()
    }
}

fn main() {
    Win::run(()).expect("Win::run failed");
}

#[cfg(test)]
mod tests {
    use gtk::LabelExt;

    use gtk_test::{assert_text, click};

    use crate::Win;

    #[test]
    fn label_change() {
        let (_component, widgets) = relm::init_test::<Win>(()).expect("init_test failed");
        let plus_button = &widgets.plus_button;
        let minus_button = &widgets.minus_button;
        let label = &widgets.counter_label;

        assert_text!(label, 0);
        click(plus_button);
        assert_text!(label, 1);
        click(plus_button);
        assert_text!(label, 2);
        click(plus_button);
        assert_text!(label, 3);
        click(plus_button);
        assert_text!(label, 4);

        click(minus_button);
        assert_text!(label, 3);
        click(minus_button);
        assert_text!(label, 2);
        click(minus_button);
        assert_text!(label, 1);
        click(minus_button);
        assert_text!(label, 0);
        click(minus_button);
        assert_text!(label, -1);
    }
}

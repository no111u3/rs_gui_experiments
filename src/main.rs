use gtk::prelude::*;
use gtk::{Inhibit, Window, WindowType};
use relm::{connect, Relm, Update, Widget};
use relm_derive::Msg;

struct Model {
    // ...
}

#[derive(Msg)]
enum Msg {
    // ...
    Quit,
}

struct Win {
    // ...
    model: Model,
    window: Window,
}

impl Update for Win {
    // Specify the model used this widget.
    type Model = Model;
    // Specify the model parameter used to init the model.
    type ModelParam = ();
    // Specify the type of the messages sent to the update function.
    type Msg = Msg;

    // Returns the initial model.
    fn model(_: &Relm<Self>, _: ()) -> Model {
        Model {}
    }

    // The model may be updated when a message is received.
    // Widgets may also be updated in this function.
    fn update(&mut self, event: Msg) {
        match event {
            Msg::Quit => gtk::main_quit(),
        }
    }
}

impl Widget for Win {
    // Specify the type of the root widget.
    type Root = Window;

    // Returns the root widget.
    fn root(&self) -> Self::Root {
        self.window.clone()
    }

    // Create the widgets.
    fn view(reml: &Relm<Self>, model: Self::Model) -> Self {
        // GTK+ widgets are used normally within a `Widget`.
        let window = Window::new(WindowType::Toplevel);

        // Connnect the signal `delete_event` to send `Quit` message.
        connect!(
            reml,
            window,
            connect_delete_event(_, _),
            return (Some(Msg::Quit), Inhibit(false))
        );
        // There is also a `connect!()` macro for GTK+ events that do not need a
        // value to be returned in the callback.

        window.show_all();

        Win { model, window }
    }
}

fn main() {
    Win::run(()).unwrap();
}

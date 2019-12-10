use std::cell::Cell;
use std::rc::Rc;

use gdk::{keyval_to_unicode, EventKey};
use gtk::{Inhibit, WidgetExt};
use relm::Widget;
use relm_derive::{widget, Msg};

use self::Msg::*;

pub struct Model {
    letter: Rc<Cell<char>>,
}

#[derive(Msg)]
pub enum Msg {
    KeyPress(EventKey),
    Quit,
}

#[widget]
impl Widget for Win {
    fn init_view(&mut self) {
        let letter = self.model.letter.clone();
        self.drawing_area.connect_draw(move |_, context| {
            context.set_source_rgb(0.2, 0.4, 0.0);
            context.paint();

            context.set_font_size(60.0);
            context.set_source_rgb(0.0, 0.0, 0.0);
            context.move_to(100.0, 100.0);
            context.show_text(&letter.get().to_string());
            Inhibit(false)
        });
    }

    fn model() -> Model {
        Model {
            letter: Rc::new(Cell::new(' ')),
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            KeyPress(event) => {
                if let Some(letter) = keyval_to_unicode(event.get_keyval()) {
                    self.model.letter.set(letter);
                    self.drawing_area.queue_draw();
                }
            }
            Quit => gtk::main_quit(),
        }
    }

    view! {
        gtk::Window {
            #[name="drawing_area"]
            gtk::DrawingArea {
            },
            delete_event(_, _) => (Quit, Inhibit(false)),
            key_press_event(_, event) => (KeyPress(event.clone()), Inhibit(false)),
        }
    }
}

fn main() {
    Win::run(()).expect("Win::run failed");
}

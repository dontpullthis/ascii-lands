
use crossterm::event::Event;

pub trait Scene {
    fn render(&self);
    fn event_handler(&mut self, event: &Event);
}
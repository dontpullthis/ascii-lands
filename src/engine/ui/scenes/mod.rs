use crossterm::event::Event;

use crate::engine::actions::Action;

pub trait Scene: Send + Sync {
    /// An initial display
    fn on_show(&self);
    fn render(&self);
    fn event_handler(&mut self, event: &Event) -> Action;
}
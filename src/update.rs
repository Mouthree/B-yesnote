use iced::{window, Task};

use crate::message::Message;
use crate::state::State;

pub fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::WindowId(id) => {
            state.window_id = id;
        }
        Message::DragWindow => {
            if let Some(id) = state.window_id {
                return window::drag(id);
            }
        }
        Message::Minimize => {
            if let Some(id) = state.window_id {
                return window::minimize(id, true);
            }
        }
        Message::ToggleMaximize => {
            if let Some(id) = state.window_id {
                return window::toggle_maximize(id);
            }
        }
        Message::Close => {
            if let Some(id) = state.window_id {
                return window::close(id);
            }
        }
    }

    Task::none()
}

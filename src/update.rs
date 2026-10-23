use iced::Task;

use crate::message::Message;
use crate::state::State;

pub fn update(_state: &mut State, _message: Message) -> Task<Message> {
    Task::none()
}

mod message;
mod state;
mod update;
mod view;

use iced::{Task, window};
use state::State;

use crate::message::Message;

fn main() -> iced::Result {
    iced::application(boot, update::update, view::view)
        .title("yesnote")
        .window(iced::window::Settings {
            size: iced::Size::new(1000.0, 700.0),
            maximized: true, 
            decorations: false,
            ..iced::window::Settings::default()
        })
        .run()
}

/// 启动获取主窗口id,用来进行窗口相关操作
fn boot() -> (State, Task<Message>) {
    (State::default(), window::latest().map(Message::WindowId))
}
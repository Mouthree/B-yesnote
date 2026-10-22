mod divider;
mod editor;
mod sidebar;
mod theme;

use divider::Divider;
use iced::widget::row;
use iced::{Element, Length, Point, Subscription, mouse};

struct App {
    divider: Divider,
}

impl Default for App {
    fn default() -> Self {
        Self {
            divider: Divider::default(),
        }
    }
}

fn main() -> iced::Result {
    iced::application(App::default, update, view)
        .subscription(subscription)
        .run()
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    CursorMoved(Point),
    ButtonPressed,
    ButtonReleased,
}

fn update(app: &mut App, message: Message) {
    match message {
        Message::CursorMoved(position) => app.divider.on_move(position.x),
        Message::ButtonPressed => app.divider.on_press(),
        Message::ButtonReleased => app.divider.on_release(),
    }
}

/// 订阅全局鼠标事件：拖动时鼠标会离开热区，局部事件会断，所以直接监听整个窗口
fn subscription(_app: &App) -> Subscription<Message> {
    iced::event::listen_with(|event, _status, _window| match event {
        iced::Event::Mouse(mouse::Event::CursorMoved { position }) => {
            Some(Message::CursorMoved(position))
        }
        iced::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
            Some(Message::ButtonPressed)
        }
        iced::Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
            Some(Message::ButtonReleased)
        }
        _ => None,
    })
}

fn view(app: &App) -> Element<'_, Message> {
    row![
        sidebar::view(app.divider.width),
        Divider::view(),
        editor::view(),
    ]
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

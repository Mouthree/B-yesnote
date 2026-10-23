use iced::widget::container;
use iced::{Color, Element, Length};

use crate::message::Message;
use crate::state::State;

const BACKGROUND: Color = Color::from_rgb8(0x28, 0x2c, 0x33);

pub fn view(_state: &State) -> Element<'_, Message> {
    container("")
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_theme| container::Style::default().background(BACKGROUND))
        .into()
}

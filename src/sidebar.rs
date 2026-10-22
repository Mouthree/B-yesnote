use crate::theme;
use crate::Message;
use iced::widget::{column, container, text};
use iced::{Element, Length};

/// 左侧小栏：宽度由分界线状态决定
pub fn view(width: f32) -> Element<'static, Message> {
    let content = column![text("YesNote").size(20), text("笔记列表").size(13)].spacing(12);

    container(content)
        .width(Length::Fixed(width))
        .height(Length::Fill)
        .padding(16)
        .style(|_| theme::surface(theme::SIDEBAR_BG, theme::TEXT_COLOR))
        .into()
}

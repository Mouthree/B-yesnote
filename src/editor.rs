use crate::Message;
use crate::theme;
use iced::widget::{column, container, text};
use iced::{Element, Length};

/// 右侧主栏：宽度自动吃掉剩余空间
pub fn view() -> Element<'static, Message> {
    let content = column![
        text("笔记标题").size(24),
        text("这里将来放笔记正文").size(14),
    ]
    .spacing(16);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(24)
        .style(|_| theme::surface(theme::MAIN_BG, theme::TEXT_COLOR))
        .into()
}

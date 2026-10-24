use iced::widget::{button, column, container, mouse_area, row, svg, text};
use iced::{Color, Element, Length};

use crate::message::Message;
use crate::state::State;

const TITLE_BAR_HEIGHT: f32 = 40.0;

const TITLE_BAR_COLOR: Color = Color::from_rgb8(0x3b, 0x41, 0x4d);
const BUTTON_HOVER: Color = Color::from_rgb8(0x4a, 0x51, 0x5e);
const BUTTON_PRESSED: Color = Color::from_rgb8(0x35, 0x3a, 0x43);
const CLOSE_HOVER: Color = Color::from_rgb8(0xe8, 0x11, 0x23);
const CLOSE_PRESSED: Color = Color::from_rgb8(0xc5, 0x0e, 0x1d);

const BACKGROUND: Color = Color::from_rgb8(0x28, 0x2c, 0x33);
const TEXT_COLOR: Color = Color::from_rgb8(0xd8, 0xde, 0xe9);

const ICON_SIZE: f32 = 18.0;

pub fn view(_state: &State) -> Element<'_, Message> {
    column![title_bar(), content()]
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn title_bar() -> Element<'static, Message> {
    let drag_area = mouse_area(
        container(text("yesnote").size(13).color(TEXT_COLOR))
            .width(Length::Fill)
            .height(Length::Fill)
            .padding([0, 12])
            .center_y(Length::Fill),
    )
    .on_press(Message::DragWindow);

    container(
        row![
            drag_area,
            window_button(
                include_bytes!("../assets/file_icons/minus.svg"),
                Message::Minimize,
                false,
            ),
            window_button(
                include_bytes!("../assets/file_icons/square.svg"),
                Message::ToggleMaximize,
                false,
            ),
            window_button(
                include_bytes!("../assets/file_icons/x.svg"),
                Message::Close,
                true,
            ),
        ]
        .height(Length::Fill),
    )
    .width(Length::Fill)
    .height(Length::Fixed(TITLE_BAR_HEIGHT))
    .style(|_theme| container::Style::default().background(TITLE_BAR_COLOR))
    .into()
}

fn content() -> Element<'static, Message> {
    container("")
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_theme| container::Style::default().background(BACKGROUND))
        .into()
}

fn icon(data: &'static [u8]) -> Element<'static, Message> {
    svg(svg::Handle::from_memory(data))
        .width(Length::Fixed(ICON_SIZE))
        .height(Length::Fixed(ICON_SIZE))
        .style(|_theme, _status| svg::Style {
            color: Some(TEXT_COLOR),
        })
        .into()
}

fn window_button(
    icon_data: &'static [u8],
    message: Message,
    danger: bool,
) -> Element<'static, Message> {
    button(
        container(icon(icon_data))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill),
    )
    .width(Length::Fixed(46.0))
    .height(Length::Fill)
    .padding(0)
    .on_press(message)
    .style(move |_theme, status| {
        let background = match status {
            button::Status::Hovered if danger => Some(CLOSE_HOVER.into()),
            button::Status::Hovered => Some(BUTTON_HOVER.into()),
            button::Status::Pressed if danger => Some(CLOSE_PRESSED.into()),
            button::Status::Pressed => Some(BUTTON_PRESSED.into()),
            _ => None,
        };

        button::Style {
            background,
            text_color: TEXT_COLOR,
            ..button::Style::default()
        }
    })
    .into()
}

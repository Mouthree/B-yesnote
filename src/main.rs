use iced::{Color, Element, Length, Size, widget::container, window::Settings};


const BACKGROUND: Color = Color::from_rgb8(0x28, 0x2c, 0x33);

#[derive(Default)]
struct State {
}
fn main() -> iced::Result{
    iced::application(State::default, update, view)
        .title("yesNote")
        .window(Settings {
            size: Size::new(1000., 700.),
            maximized: true,
            ..Settings::default()
        })
        .run()
}

#[derive(Debug, Clone)]
enum Message{}

fn update(_state: &mut State, _message: Message) {
    
}

fn view(_state: &State) -> Element<'_, Message> {
    container("")
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_theme| container::Style::default().background(BACKGROUND))
        .into()
}
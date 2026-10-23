mod message;
mod state;
mod update;
mod view;

use state::State;

fn main() -> iced::Result {
    iced::application(State::default, update::update, view::view)
        .title("yesnote")
        .window(iced::window::Settings {
            size: iced::Size::new(1000.0, 700.0),
            maximized: true,
            ..iced::window::Settings::default()
        })
        .run()
}
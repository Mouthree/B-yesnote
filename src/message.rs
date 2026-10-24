use iced::window;

#[derive(Debug, Clone)]
pub enum Message {
    WindowId(Option<window::Id>),
    DragWindow,
    Minimize,
    ToggleMaximize,
    Close,
}
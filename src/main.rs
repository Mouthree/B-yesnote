use iced::widget::{column, container, mouse_area, row, text};
use iced::{mouse, Color, Element, Length, Point, Subscription};

// ===== 可调参数 =====

/// 侧栏默认宽度
const DEFAULT_SIDEBAR_WIDTH: f32 = 240.0;
/// 侧栏允许的最小宽度
const MIN_SIDEBAR_WIDTH: f32 = 160.0;
/// 侧栏允许的最大宽度
const MAX_SIDEBAR_WIDTH: f32 = 420.0;

/// 拖动热区宽度。视觉上不画线，分界线两侧各占一半，都算可拖动范围
const DIVIDER_HIT_WIDTH: f32 = 14.0;

/// 左侧小栏背景色 #282828
const SIDEBAR_BG: Color = Color::from_rgb8(0x28, 0x28, 0x28);
/// 右侧主栏背景色 #1C1C1C
const MAIN_BG: Color = Color::from_rgb8(0x1C, 0x1C, 0x1C);
/// 深色背景上的文字颜色
const TEXT_COLOR: Color = Color::from_rgb8(0xE5, 0xE5, 0xE5);

struct App {
    /// 左栏的实际宽度
    sidebar_width: f32,
    /// 鼠标在窗口中的横向位置
    cursor_x: f32,
    /// 是否正在拖动分界线
    dragging: bool,
    /// 按下时光标相对分界线的偏移，拖动时保持这个偏移，手感才不跳
    drag_offset: f32,
}

impl Default for App {
    fn default() -> Self {
        Self {
            sidebar_width: DEFAULT_SIDEBAR_WIDTH,
            cursor_x: 0.0,
            dragging: false,
            drag_offset: 0.0,
        }
    }
}

fn main() -> iced::Result {
    iced::application(App::default, update, view)
        .subscription(subscription)
        .run()
}

#[derive(Debug, Clone, Copy)]
enum Message {
    CursorMoved(Point),
    ButtonPressed,
    ButtonReleased,
}

fn update(state: &mut App, message: Message) {
    match message {
        Message::CursorMoved(position) => {
            state.cursor_x = position.x;

            if state.dragging {
                // 分界线的新位置 = 光标位置 - 按下时的偏移
                let boundary = position.x - state.drag_offset;
                // 状态里存的是左栏宽度，而分界线在热区正中间，所以要减去半个热区
                let width = boundary - DIVIDER_HIT_WIDTH / 2.0;
                state.sidebar_width = width.clamp(MIN_SIDEBAR_WIDTH, MAX_SIDEBAR_WIDTH);
            }
        }
        Message::ButtonPressed => {
            // 视觉分界线在“左栏宽度 + 半个热区”的位置
            let boundary = state.sidebar_width + DIVIDER_HIT_WIDTH / 2.0;

            // 落在热区内才开始拖动
            if (state.cursor_x - boundary).abs() <= DIVIDER_HIT_WIDTH / 2.0 {
                state.dragging = true;
                state.drag_offset = state.cursor_x - boundary;
            }
        }
        Message::ButtonReleased => {
            state.dragging = false;
        }
    }
}

/// 订阅全局鼠标事件：拖动时鼠标会离开热区，局部事件会断，所以直接监听整个窗口
fn subscription(_state: &App) -> Subscription<Message> {
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

fn view(state: &App) -> Element<'_, Message> {
    row![
        sidebar(state.sidebar_width),
        divider(),
        main_panel(),
    ]
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

/// 左侧小栏：宽度由状态决定
fn sidebar(width: f32) -> Element<'static, Message> {
    let content = column![
        text("YesNote").size(20),
        text("笔记列表").size(13),
    ]
    .spacing(12);

    container(content)
        .width(Length::Fixed(width))
        .height(Length::Fill)
        .padding(16)
        .style(|_theme| {
            container::Style::default()
                .background(SIDEBAR_BG)
                .color(TEXT_COLOR)
        })
        .into()
}

/// 可拖动的分界区域：不画任何线。
///
/// 左半涂左栏颜色、右半涂右栏颜色，和两侧无缝拼接，看起来就是两块颜色直接相接；
/// 但整个 14px 宽的条都是鼠标热区，悬停时光标变成左右拖动样式。
fn divider() -> Element<'static, Message> {
    let half = DIVIDER_HIT_WIDTH / 2.0;

    // 左半：延续左栏底色
    let left_half = container("")
        .width(Length::Fixed(half))
        .height(Length::Fill)
        .style(|_theme| container::Style::default().background(SIDEBAR_BG));

    // 右半：延续右栏底色
    let right_half = container("")
        .width(Length::Fixed(half))
        .height(Length::Fill)
        .style(|_theme| container::Style::default().background(MAIN_BG));

    let hit_area = row![left_half, right_half].height(Length::Fill);

    mouse_area(hit_area)
        .interaction(mouse::Interaction::ResizingColumn)
        .into()
}

/// 右侧主栏：宽度自动吃掉剩余空间
fn main_panel() -> Element<'static, Message> {
    let content = column![
        text("笔记标题").size(24),
        text("这里将来放笔记正文").size(14),
    ]
    .spacing(16);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(24)
        .style(|_theme| {
            container::Style::default()
                .background(MAIN_BG)
                .color(TEXT_COLOR)
        })
        .into()
}

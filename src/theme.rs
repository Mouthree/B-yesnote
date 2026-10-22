use iced::Color;
use iced::widget::container;

// ===== 可调参数 =====

/// 侧栏默认宽度
pub const DEFAULT_SIDEBAR_WIDTH: f32 = 240.0;
/// 侧栏允许的最小宽度
pub const MIN_SIDEBAR_WIDTH: f32 = 160.0;
/// 侧栏允许的最大宽度
pub const MAX_SIDEBAR_WIDTH: f32 = 420.0;

/// 拖动热区宽度。视觉上不画线，分界线两侧各占一半，都算可拖动范围
pub const DIVIDER_HIT_WIDTH: f32 = 14.0;

/// 左侧小栏背景色 #282828
pub const SIDEBAR_BG: Color = Color::from_rgb8(0x28, 0x28, 0x28);
/// 右侧主栏背景色 #1C1C1C
pub const MAIN_BG: Color = Color::from_rgb8(0x1C, 0x1C, 0x1C);
/// 深色背景上的文字颜色
pub const TEXT_COLOR: Color = Color::from_rgb8(0xE5, 0xE5, 0xE5);

// ===== 可复用样式 =====

/// 只有底色的容器样式，用于分界线两侧的无缝拼接
pub fn background(color: Color) -> container::Style {
    container::Style::default().background(color)
}

/// 同时设置底色和文字颜色的容器样式，用于左右两栏
pub fn surface(bg: Color, fg: Color) -> container::Style {
    container::Style::default().background(bg).color(fg)
}

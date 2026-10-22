use crate::theme;
use crate::Message;
use iced::widget::{container, mouse_area, row};
use iced::{mouse, Element, Length};

/// 左右分栏的可拖动分界线：宽度、指针位置和拖动状态都在这里，交互不散落到 update
pub struct Divider {
    /// 左栏的实际宽度
    pub width: f32,
    /// 鼠标在窗口中的横向位置
    cursor_x: f32,
    /// 是否正在拖动分界线
    dragging: bool,
    /// 按下时光标相对分界线的偏移，拖动时保持这个偏移，手感才不跳
    drag_offset: f32,
}

impl Default for Divider {
    fn default() -> Self {
        Self {
            width: theme::DEFAULT_SIDEBAR_WIDTH,
            cursor_x: 0.0,
            dragging: false,
            drag_offset: 0.0,
        }
    }
}

impl Divider {
    /// 视觉分界线在“左栏宽度 + 半个热区”的位置
    fn boundary(&self) -> f32 {
        self.width + theme::DIVIDER_HIT_WIDTH / 2.0
    }

    pub fn on_press(&mut self) {
        let boundary = self.boundary();

        // 落在热区内才开始拖动
        if (self.cursor_x - boundary).abs() <= theme::DIVIDER_HIT_WIDTH / 2.0 {
            self.dragging = true;
            self.drag_offset = self.cursor_x - boundary;
        }
    }

    pub fn on_move(&mut self, x: f32) {
        self.cursor_x = x;

        if self.dragging {
            // 分界线的新位置 = 光标位置 - 按下时的偏移
            let boundary = x - self.drag_offset;
            // 状态里存的是左栏宽度，而分界线在热区正中间，所以要减去半个热区
            let width = boundary - theme::DIVIDER_HIT_WIDTH / 2.0;
            self.width = width.clamp(theme::MIN_SIDEBAR_WIDTH, theme::MAX_SIDEBAR_WIDTH);
        }
    }

    pub fn on_release(&mut self) {
        self.dragging = false;
    }

    /// 可拖动的分界区域：不画任何线。
    ///
    /// 左半涂左栏颜色、右半涂右栏颜色，和两侧无缝拼接，看起来就是两块颜色直接相接；
    /// 但整个 14px 宽的条都是鼠标热区，悬停时光标变成左右拖动样式。
    pub fn view() -> Element<'static, Message> {
        let half = theme::DIVIDER_HIT_WIDTH / 2.0;

        // 左半：延续左栏底色
        let left_half = container("")
            .width(Length::Fixed(half))
            .height(Length::Fill)
            .style(|_| theme::background(theme::SIDEBAR_BG));

        // 右半：延续右栏底色
        let right_half = container("")
            .width(Length::Fixed(half))
            .height(Length::Fill)
            .style(|_| theme::background(theme::MAIN_BG));

        let hit_area = row![left_half, right_half].height(Length::Fill);

        mouse_area(hit_area)
            .interaction(mouse::Interaction::ResizingColumn)
            .into()
    }
}

use ggez::graphics::Rect;

#[derive(Clone, Copy)]
pub struct LayoutInfo {
    /// The rectangle inside the parent.
    pub relative_rect: Rect,
}

impl LayoutInfo {
    pub fn get_absolute_rect(&self, parent_absolute_rect: &Rect) -> Rect {
        Rect {
            x: self.relative_rect.x + parent_absolute_rect.x,
            y: self.relative_rect.y + parent_absolute_rect.y,
            w: self.relative_rect.w,
            h: self.relative_rect.h,
        }
    }

    pub(crate) fn new() -> LayoutInfo {
        LayoutInfo {
            relative_rect: Rect::new(0., 0., 0., 0.),
        }
    }
}

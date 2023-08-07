use ggez::{graphics::Rect, mint::Point2};

#[derive(Clone, Copy)]
pub struct LayoutInfo {
    /// The rectangle inside the parent.
    pub raw_rect_in_parent: Rect,

    /// Where \[0.0, 0.0] coordinates are in the parent.
    /// 0.0 = top/left, 0.5 = center, 1.0 = bottom/right
    pub anchor: Point2<f32>,

    /// Where \[0.0, 0.0] coordinates are in this drawable.
    /// 0.0 = top/left, 0.5 = center, 1.0 = bottom/right
    pub origin: Point2<f32>,

    /// Multiplies the size of this drawable. 1.0 by default
    pub scale: Point2<f32>,

    /// Whether width or height in raw_rect_in_parent
    /// are expressed in percentages of parent size.
    pub relative_size: Point2<bool>,
}

impl LayoutInfo {
    pub fn get_absolute_rect(&self, parent_screen_rect: &Rect) -> Rect {
        // scale position and size to match parent scaling
        let mut x = self.raw_rect_in_parent.x;
        let mut y = self.raw_rect_in_parent.y;
        let mut w = self.raw_rect_in_parent.w;
        let mut h = self.raw_rect_in_parent.h;

        // adjust for relative sizes
        if self.relative_size.x {
            w *= parent_screen_rect.w;
        }
        if self.relative_size.y {
            h *= parent_screen_rect.h;
        }

        // scale this drawable (around its origin)
        w *= self.scale.x;
        h *= self.scale.y;

        // adjust for origin
        x -= self.origin.x * w;
        y -= self.origin.y * h;

        Rect { x, y, w, h }
    }

    pub fn get_screen_rect(&self, parent_screen_rect: &Rect, parent_rect_scale: f32) -> Rect {
        // scale position and size to match parent scaling
        let mut x = (self.raw_rect_in_parent.x * parent_rect_scale) + parent_screen_rect.x;
        let mut y = (self.raw_rect_in_parent.y * parent_rect_scale) + parent_screen_rect.y;
        let mut w = self.raw_rect_in_parent.w * parent_rect_scale;
        let mut h = self.raw_rect_in_parent.h * parent_rect_scale;

        // adjust for relative sizes
        if self.relative_size.x {
            w *= parent_screen_rect.w;
        }
        if self.relative_size.y {
            h *= parent_screen_rect.h;
        }

        // adjust for anchor
        x += self.anchor.x * parent_screen_rect.w;
        y += self.anchor.y * parent_screen_rect.h;

        // scale this drawable (around its origin)
        w *= self.scale.x;
        h *= self.scale.y;

        // adjust for origin
        x -= self.origin.x * w;
        y -= self.origin.y * h;

        Rect { x, y, w, h }
    }

    pub fn new() -> LayoutInfo {
        LayoutInfo {
            raw_rect_in_parent: Rect::new(0., 0., 1., 1.),
            anchor: Point2 { x: 0.0, y: 0.0 },
            origin: Point2 { x: 0.0, y: 0.0 },
            scale: Point2 { x: 1.0, y: 1.0 },
            relative_size: Point2 { x: false, y: false },
        }
    }

    pub fn new_centered() -> LayoutInfo {
        LayoutInfo {
            raw_rect_in_parent: Rect::new(0., 0., 1., 1.),
            anchor: Point2 { x: 0.5, y: 0.5 },
            origin: Point2 { x: 0.5, y: 0.5 },
            scale: Point2 { x: 1.0, y: 1.0 },
            relative_size: Point2 { x: false, y: false },
        }
    }
}

impl Default for LayoutInfo {
    fn default() -> Self {
        Self::new()
    }
}

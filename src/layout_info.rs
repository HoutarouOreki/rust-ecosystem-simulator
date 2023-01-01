use ggez::{graphics::Rect};

#[derive(Clone, Copy)]
pub struct LayoutInfo {
    /// The rectangle inside the parent.
    pub relative_rect: Rect,
}

impl LayoutInfo {
    pub fn get_absolute_rect(&self, parent_absolute_rect: &Rect, parent_rect_scale: f32) -> Rect {
        // let rect = self.relative_rect.clone();

        // // first, our rectangle will be positioned as if parent's top_left was the <0, 0> point
        // rect.translate(vecmath::vec2_neg(parent_absolute_rect.point().into()));

        // // then, we scale ourselves up (towards bottom right) to match the scaling of parent
        // rect.scale(parent_rect_scale, parent_rect_scale);

        // // then we go back to original position
        // rect.

        let x = (self.relative_rect.x * parent_rect_scale) + parent_absolute_rect.x;
        let y = (self.relative_rect.y * parent_rect_scale) + parent_absolute_rect.y;

        // let mut rect = self.relative_rect;
        // rect.scale(parent_rect_scale, parent_rect_scale);
        // let Point2 { x: original_x, y: original_y } = rect.point();
        // rect.translate(offset * parent_rect_scale);

        Rect {
            x,
            y,
            w: self.relative_rect.w * parent_rect_scale,
            h: self.relative_rect.h * parent_rect_scale,
        }
    }

    pub(crate) fn new() -> LayoutInfo {
        LayoutInfo {
            relative_rect: Rect::new(0., 0., 0., 0.),
        }
    }
}

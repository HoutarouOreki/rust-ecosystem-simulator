use ggez::graphics::{DrawParam, Rect, Text};

use crate::layout_info::LayoutInfo;

use super::{organism::Organism, states::shared_state::SharedState};

#[derive(Clone)]
pub struct OrganismInfo {
    layout_info: LayoutInfo,
    shared_state: SharedState,
    info_text: Text,
}

impl OrganismInfo {
    pub fn new(organism: &Organism) -> Self {
        OrganismInfo {
            layout_info: organism.layout_info,
            shared_state: organism.shared_state.to_owned(),
            info_text: organism.info_text.to_owned(),
        }
    }

    pub fn from_organisms(organisms: &Vec<Organism>) -> Vec<OrganismInfo> {
        let mut vec = Vec::new();
        vec.reserve(organisms.len());
        for organism in organisms {
            vec.push(OrganismInfo::new(organism));
        }
        vec
    }

    pub fn from_organisms_fill_vec(organisms: &Vec<Organism>, target: &mut Vec<OrganismInfo>) {
        target.clear();
        target.reserve(organisms.len());
        for organism in organisms {
            target.push(OrganismInfo::new(organism))
        }
    }

    pub fn get_draw_param(
        &self,
        parent_screen_rect: &Rect,
        parent_rect_scale: f32,
        visibility_rect: &Rect,
    ) -> Option<DrawParam> {
        let screen_rect = self
            .layout_info
            .get_screen_rect(parent_screen_rect, parent_rect_scale);

        if !screen_rect.overlaps(visibility_rect) {
            return Option::None;
        }

        Some(
            DrawParam::default()
                .dest_rect(screen_rect)
                .color(self.shared_state.species.color),
        )
    }
}

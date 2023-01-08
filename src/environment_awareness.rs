use std::collections::HashMap;

use ggez::{graphics::Rect, mint::Point2};

use crate::organisms::{organism::Organism, states::organism_state::ForeignerInfo};

pub struct EnvironmentAwareness {
    chunk_size: f32,
    chunks: HashMap<Point2<i32>, Vec<ForeignerInfo>>,
}

impl EnvironmentAwareness {
    pub fn new(chunk_size: f32) -> Self {
        Self {
            chunk_size,
            chunks: HashMap::new(),
        }
    }

    pub fn refill(&mut self, organisms: &Vec<Organism>) {
        for chunk in self.chunks.values_mut() {
            chunk.clear();
        }

        for organism in organisms {
            let chunk = self.get_chunk_on_point_mut(organism.position());
            chunk.push(ForeignerInfo::new(organism));
        }
    }

    pub fn get_chunks(&self) -> &HashMap<Point2<i32>, Vec<ForeignerInfo>> {
        &self.chunks
    }

    pub fn get_chunk_coordinates(&self, index: Point2<i32>) -> Rect {
        let index_f32 = Point2 {
            x: index.x as f32,
            y: index.y as f32,
        };
        let center = Point2 {
            x: index_f32.x * self.chunk_size,
            y: index_f32.y * self.chunk_size,
        };

        let half_size = self.chunk_size * 0.5;
        Rect {
            x: center.x - half_size,
            y: center.y - half_size,
            w: self.chunk_size,
            h: self.chunk_size,
        }
    }

    pub fn get_chunks_in_rect(&self, rect: Rect) -> impl Iterator<Item = &ForeignerInfo> + '_ {
        let (left, top, right, bottom) = self.left_top_right_bottom_indexes(rect);

        (top..=bottom)
            .flat_map(move |y| (left..=right).map(move |x| (x, y)))
            .filter_map(|(x, y)| self.chunks.get(&Point2 { x, y }))
            .flatten()
    }

    fn left_top_right_bottom_indexes(&self, world_coordinates_rect: Rect) -> (i32, i32, i32, i32) {
        let Point2 { x: left, y: top } = self.get_chunk_index_on_point(Point2 {
            x: world_coordinates_rect.left(),
            y: world_coordinates_rect.top(),
        });
        let Point2 {
            x: right,
            y: bottom,
        } = self.get_chunk_index_on_point(Point2 {
            x: world_coordinates_rect.right(),
            y: world_coordinates_rect.bottom(),
        });
        (left, top, right, bottom)
    }

    pub fn get_chunk_index_on_point(&self, point: Point2<f32>) -> Point2<i32> {
        let half_size = self.chunk_size * 0.5;
        let x = ((point.x + half_size) / self.chunk_size).floor() as i32;
        let y = ((point.y + half_size) / self.chunk_size).floor() as i32;
        Point2 { x, y }
    }

    pub fn get_chunk_on_point(&mut self, point: Point2<f32>) -> &Vec<ForeignerInfo> {
        let index = self.get_chunk_index_on_point(point);
        self.chunks.entry(index).or_insert_with(Vec::new)
    }

    pub fn get_chunk_on_point_mut(&mut self, point: Point2<f32>) -> &mut Vec<ForeignerInfo> {
        let index = self.get_chunk_index_on_point(point);
        self.chunks.entry(index).or_insert_with(Vec::new)
    }

    pub fn get_radius_around(
        &self,
        position: Point2<f32>,
        radius: f32,
    ) -> impl Iterator<Item = &ForeignerInfo> {
        self.get_chunks_in_rect(Rect {
            x: position.x - radius,
            y: position.y - radius,
            w: radius * 2.0,
            h: radius * 2.0,
        })
    }
}

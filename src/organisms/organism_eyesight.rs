use ggez::mint::Point2;

use super::organism::Organism;

pub struct OrganismEyesight {
    host: Box<Organism>,
    organisms: Vec<Organism>,
}

impl OrganismEyesight {
    pub fn see_organisms(&self) -> Vec<&Organism> {
        let mut vec: Vec<&Organism> = Vec::new();
        for organism in self.organisms.iter() {
            if organism.id() == self.host.id() {
                continue;
            }
            if Self::distance(&self.host.position(), &organism.position()) > 10.0 {
                continue;
            }
            vec.push(organism);
        }
        vec
    }

    fn distance(a: &Point2<f32>, b: &Point2<f32>) -> f32 {
        let width = a.x - b.x;
        let height = a.y - b.y;
        ((width * width) + (height * height)).sqrt()
    }
}

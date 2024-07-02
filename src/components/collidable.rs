use crate::collider::Hitbox;

pub struct Collidable {
    hitbox: Hitbox,
}

impl Collidable {
    pub fn new(hitbox: Hitbox) -> Self {
        Self { hitbox }
    }

    pub fn hitbox(&self) -> &Hitbox {
        &self.hitbox
    }
}

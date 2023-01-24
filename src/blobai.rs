use macroquad::rand;
use crate::v2d::Vector2D;
use crate::blob::Blob;

pub struct BlobAI
{
    blob: Blob,
    target: Vector2D
}

impl BlobAI
{
    pub fn new(blob: Blob) -> BlobAI
    {
        let pos = blob.position;
        return BlobAI
        {
            blob: blob,
            target: pos
        };
    }

    pub fn draw(&self)
    {
        self.blob.draw();
    }

    pub fn update(&mut self, dt: f32)
    {
        let dist = Vector2D::distance(self.target, self.blob.position);
        if dist < 0.01
        {
            self.target.x = rand::gen_range(self.blob.position.x - 0.5, self.blob.position.x + 0.5);
            self.target.y = rand::gen_range(self.blob.position.y - 0.5, self.blob.position.y + 0.5);
        }

        self.blob.update(dt, self.target);
    }
}
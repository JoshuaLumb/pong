pub mod entity {

    use tetra::graphics::{Rectangle, Texture};
    use tetra::math::Vec2;

    pub struct Entity {
        // An entity here will be any stateful object in the game
        pub texture: Texture,
        pub position: Vec2<f32>,
        pub velocity: Vec2<f32>,
    }

    impl Entity {
        /* Entities will have a texture and a position on the map
        * as well as bounds with which collision is detected */
        pub fn new(texture: Texture, position: Vec2<f32>) -> Entity {
            Entity::with_velocity(texture, position, Vec2::zero())
        }

        pub fn with_velocity(texture: Texture, position: Vec2<f32>, velocity: Vec2<f32>) -> Entity {
            Entity {
                texture,
                position,
                velocity,
            }
        }

        pub fn width(&self) -> f32 {
            self.texture.width() as f32
        }

        pub fn height(&self) -> f32 {
            self.texture.height() as f32
        }

        pub fn bounds(&self) -> Rectangle {
            Rectangle::new(
                self.position.x,
                self.position.y,
                self.width(),
                self.height(),
            )
        }
    }
}

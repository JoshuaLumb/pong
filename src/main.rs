use tetra::graphics::{self, Color, Texture};
use tetra::input::{self, Key};
use tetra::math::Vec2;
use tetra::{Context, ContextBuilder, State};

const WINDOW_WIDTH: f32 = 640.0;
const WINDOW_HEIGHT: f32 = 480.0;
const PADDLE_SPEED: f32 = 8.0;

struct GameState {
    /* GameState is a struct implementing Tetra's State trait.
     * Here we'll hold relevant values for stateful game elements. */
    paddle_texture: Texture,
    paddle_position: Vec2<f32>
}

impl GameState{
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let paddle_texture = Texture::new(ctx, "./resources/player1.png")?;
        let paddle_position = Vec2::new(16.0, (WINDOW_HEIGHT - paddle_texture.height() as f32) / 2.0);
        Ok(GameState {
            paddle_texture,
            paddle_position,
        } )
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result{
        //Set up a blank screen
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));

        //Draw Player1 Paddle
        graphics::draw(ctx, &self.paddle_texture, self.paddle_position);

        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        // Tetra updates state 60 times per second by default.

        //Handle inputs for player 1
        if input::is_key_down(ctx, Key::W) && self.paddle_position.y > 0.0 {
            self.paddle_position.y -= PADDLE_SPEED;
        }
        if input::is_key_down(ctx, Key::S) && self.paddle_position.y < (WINDOW_HEIGHT - self.paddle_texture.height() as f32) {
            self.paddle_position.y += PADDLE_SPEED;
        }

        Ok(())
    }
}

fn main() -> tetra::Result {
    /* Establish a context - Tetra uses this to hold global state info like window settings
     * and connections to graphics/audio/input hardware. */
    ContextBuilder::new("Pong", WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}

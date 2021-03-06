mod game_entities;

use tetra::graphics::{self, Color, Texture};
use tetra::input::{self, Key};
use tetra::math::Vec2;
use tetra::window;
use tetra::{Context, ContextBuilder, State};
use game_entities::entity::{Entity};

const WINDOW_WIDTH: f32 = 640.0;
const WINDOW_HEIGHT: f32 = 480.0;
const PADDLE_SPEED: f32 = 8.0;
const BALL_SPEED: f32 = 5.0;
const PADDLE_SPIN: f32 = 4.0;
const BALL_ACC: f32 = 0.05;

struct GameState {
    /* GameState is a struct implementing Tetra's State trait.
     * Here we'll hold relevant values for stateful game elements. */
    player1: Entity,
    player2: Entity,
    ball: Entity,
}

impl GameState{
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let player1_texture = Texture::new(ctx, "./resources/player1.png")?;
        let player1_position = Vec2::new(
            16.0,
            (WINDOW_HEIGHT - player1_texture.height() as f32) / 2.0,
        );

        let player2_texture = Texture::new(ctx, "./resources/player2.png")?;
        let player2_position = Vec2::new(
            WINDOW_WIDTH - 16.0 - player2_texture.width() as f32,
            (WINDOW_HEIGHT - player2_texture.height() as f32) / 2.0,
        );

        let ball_texture = Texture::new(ctx, "./resources/ball.png")?;
        let ball_position = Vec2::new(
            WINDOW_WIDTH / 2.0 - ball_texture.width() as f32 / 2.0,
            WINDOW_HEIGHT /2.0 - ball_texture.height() as f32 / 2.0,
        );

        let ball_velocity = Vec2::new(-BALL_SPEED, 0.0);

        Ok(GameState {
            player1: Entity::new(player1_texture, player1_position),
            player2: Entity::new(player2_texture, player2_position),
            ball: Entity::with_velocity(ball_texture, ball_position, ball_velocity),
        })
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result{
        //Set up a blank screen
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));

        //Draw player paddles
        graphics::draw(ctx, &self.player1.texture, self.player1.position);
        graphics::draw(ctx, &self.player2.texture, self.player2.position);
        graphics::draw(ctx, &self.ball.texture, self.ball.position);

        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        // Tetra updates state 60 times per second by default.

        // Handle inputs for player 1
        if input::is_key_down(ctx, Key::W) && self.player1.position.y > 0.0 {
            self.player1.position.y -= PADDLE_SPEED;
        }
        if input::is_key_down(ctx, Key::S) && self.player1.position.y < (WINDOW_HEIGHT - self.player1.texture.height() as f32) {
            self.player1.position.y += PADDLE_SPEED;
        }
        // ...and player 2
        if input::is_key_down(ctx, Key::Up) && self.player2.position.y > 0.0{
            self.player2.position.y -= PADDLE_SPEED;
        }
        if input::is_key_down(ctx, Key::Down) && self.player2.position.y < (WINDOW_HEIGHT - self.player2.texture.height() as f32) {
            self.player2.position.y += PADDLE_SPEED;
        }

        // Handle ball position
        self.ball.position += self.ball.velocity;

        // Handle collisions
        let player1_bounds = self.player1.bounds();
        let player2_bounds = self.player2.bounds();
        let ball_bounds = self.ball.bounds();

        // Tetra has a built in "intersects" method for collision detection
        let paddle_hit = 
            if ball_bounds.intersects(&player1_bounds) {
                Some(&self.player1)
            } else if ball_bounds.intersects(&player2_bounds) {
                Some(&self.player2)
            } else{
                None
            };

        // Add a bit of "spin" based offset from the struck paddle's centre. 
        if let Some(paddle) = paddle_hit {
            self.ball.velocity.x = -(self.ball.velocity.x + (BALL_ACC * self.ball.velocity.x.signum()));
        
            let offset = (paddle.centre().y - self.ball.centre().y) / paddle.height();

            self.ball.velocity.y += PADDLE_SPIN * -offset;
        }

        if self.ball.position.y <= 0.0 || self.ball.position.y + self.ball.height() >= WINDOW_HEIGHT {
            self.ball.velocity.y = -self.ball.velocity.y;
        }

        //Win conditions!
        if self.ball.position.x < 0.0 {
            window::quit(ctx);
            println!("Player 2 wins!");
        }
        
        if self.ball.position.x > WINDOW_WIDTH {
            window::quit(ctx);
            println!("Player 1 wins!");
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

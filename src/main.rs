use macroquad::prelude::*;

mod v2d;
use v2d::Vector2D;

mod blob;
use blob::Blob;

mod blobai;
use blobai::BlobAI;

mod food;
use food::FoodArray;



//gamestate
struct GameState
{
    blobs: Vec<BlobAI>,
    foods: FoodArray,
    player: Blob,
}

impl GameState
{
    fn new() -> GameState
    {
        return GameState 
        {
            blobs: Vec::with_capacity(10),
            foods: FoodArray::new(),
            player: Blob::from_xy(0.0, 0.0, YELLOW)
        };
    }

    fn draw(&self) 
    {
        self.player.draw();
        for blob in &self.blobs 
        {
            blob.draw();
        }
        self.foods.draw();
    }

    fn update(&mut self, dt: f32)
    {
        self.player.update(dt, Vector2D::new());
        for blob in &mut self.blobs
        {
            blob.update(dt);
        }
    }
}

#[macroquad::main("Agar")]
async fn main() 
{
    request_new_screen_size(1600.0, 900.0);
    
    let mut target = (0.0, 0.0);
    let mut zoom = 1.0;

    let mut dt: f32 = 0.0;

    let mut game = GameState::new();

    for _i in 0..30
    {
        let x = rand::gen_range(-10.0, 10.0);
        let y = rand::gen_range(-10.0, 10.0);

        let blob = Blob::from_xy(x, y, RED);
        let ai = BlobAI::new(blob);
        game.blobs.push(ai);
    }

    loop 
    {
        //camera controls
        if is_key_down(KeyCode::W) 
        {
            target.1 += 0.1;
        }
        if is_key_down(KeyCode::S) 
        {
            target.1 -= 0.1;
        }
        if is_key_down(KeyCode::A) 
        {
            target.0 -= 0.1;
        }
        if is_key_down(KeyCode::D) 
        {
            target.0 += 0.1;
        }

        if is_key_down(KeyCode::Escape) 
        {
            break;
        }

        match mouse_wheel() 
        {
            (_x, y) if y != 0.0 => 
            {
                zoom *= 1.05f32.powf(y);
            }
            _ => (),
        }


        clear_background(WHITE);

        set_camera(&Camera2D {
            target: vec2(target.0, target.1),
            zoom: vec2(zoom, zoom * screen_width() / screen_height()),
            ..Default::default()
        }); 

        game.draw();

        dt = get_frame_time();
        game.update(dt);

        // Back to screen space, render some text
        set_default_camera();
        draw_text(
            format!("target (WASD keys) = ({:+.2}, {:+.2})", target.0, target.1).as_str(),
            10.0,
            10.0,
            15.0,
            BLACK,
        );
        draw_text(
            format!("zoom (mouse wheel) = {:.2}", zoom).as_str(),
            10.0,
            25.0,
            15.0,
            BLACK,
        );
        draw_text(
            format!("mouse pos = ({:.2}, {:+.2})", mouse_position().0, mouse_position().1).as_str(),
            10.0,
            40.0,
            15.0,
            BLACK,
        );

        next_frame().await
    }
}

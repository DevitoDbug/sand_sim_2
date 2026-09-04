use macroquad::prelude::*;
use macroquad::{
    rand::gen_range,
    color::Color,
};
use crate::engine::consts::{BLOCK_SIZE, ROWS, COLS};

struct SandParticle {
    x: f32, 
    y: f32,
    color: Color,
}

pub struct Game {
    particles: Vec<SandParticle>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            particles: vec![],
        }
    }

    pub async fn render(&mut self) {
        let layers = vec![RED, ORANGE, GREEN, GOLD, SKYBLUE, PURPLE];
        let mut current_layer_index = 0;
        let mut layer_timer = 0.0;
        let layer_duration = 5.0;

        loop {
            clear_background(WHITE);

            if is_mouse_button_down(MouseButton::Left){
                self.spawn_particle(layers[current_layer_index]);
            }

            self.draw_all_particles();
            self.drop_particles();

            layer_timer += get_frame_time();
            if layer_timer >= layer_duration {
                layer_timer = 0.;

                current_layer_index = (current_layer_index + 1) % layers.len();
            }
            next_frame().await;
        }
    }

    fn draw_all_particles(&self) {
        for particle in &self.particles{
            draw_rectangle(
                particle.x,
                particle.y, 
                BLOCK_SIZE,
                BLOCK_SIZE, 
                particle.color,
            );
        }
    }

    fn drop_particles(&mut self) {
        for i in 0..self.particles.len(){
            let particle = &self.particles[i];
            if self.can_move_down(&particle) {
                self.particles[i].y += BLOCK_SIZE;
            }else {
               let dx = if gen_range(0,2) == 0 { 1 } else {-1};
               if self.can_move_down_sideways(&particle, dx) {
                   self.particles[i].x += BLOCK_SIZE * dx as f32;
                   self.particles[i].y += BLOCK_SIZE;
               }
            }
        }
    }

    fn can_move_down(&self, target_particle: &SandParticle)->bool {
        if target_particle.y + BLOCK_SIZE >= ROWS as f32 * BLOCK_SIZE {
            return false;
        }

        for particle in &self.particles{
            if  particle.x == target_particle.x &&  
                particle.y == target_particle.y + BLOCK_SIZE {
                return false;
            }
        }

        true
    }

    fn can_move_down_sideways(&self, target_particle: &SandParticle, dx: i32)->bool {
        let screen_width = COLS as f32 * BLOCK_SIZE;
        let screen_height = ROWS as f32 * BLOCK_SIZE;
        let dx = BLOCK_SIZE * dx as f32;
        if target_particle.x + dx > screen_width ||
          target_particle.x + dx < 0.{
                return false;
        }

        if target_particle.y + BLOCK_SIZE >= screen_height{
            return false;
        }

        for particle in &self.particles {
            if particle.x == target_particle.x + dx  &&  
                particle.y == target_particle.y + BLOCK_SIZE {
                return false;
            }
        }

        true
    }

    fn spawn_particle(&mut self, color: Color) {
        let (x, y) = mouse_position();
        let (x, y) = (
                x as i32 / BLOCK_SIZE as i32 * BLOCK_SIZE as i32,
                y as i32/ BLOCK_SIZE as i32 * BLOCK_SIZE as i32
            );
        for i in -1..1 {
            for k in -1..1 {
                self.particles.push( SandParticle {
                        x: x as f32 + (i as f32 * BLOCK_SIZE) ,
                        y: y as f32 + (k as f32 * BLOCK_SIZE),
                        color
                });
            }
        }
    }
}



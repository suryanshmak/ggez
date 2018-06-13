//! A very simple shader example.

#[macro_use]
extern crate gfx;
extern crate ggez;

use ggez::conf;
use ggez::event;
use ggez::filesystem;
use ggez::graphics::{self, DrawMode, Point2};
use ggez::timer;
use ggez::{Context, GameResult};
use std::env;
use std::path;

gfx_defines!{
    constant Dim {
        rate: f32 = "u_Rate",
    }
}

struct MainState {
    dim: Dim,
    shader: graphics::Shader<Dim>,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let dim = Dim { rate: 0.5 };
        let shader = graphics::Shader::new(
            ctx,
            "/basic_150.glslv",
            "/dimmer_150.glslf",
            dim,
            "Dim",
            None,
        )?;
        Ok(MainState { dim, shader })
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.dim.rate = 0.5 + (((timer::get_ticks(ctx) as f32) / 100.0).cos() / 2.0);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        graphics::circle(
            ctx,
            graphics::WHITE,
            DrawMode::Fill,
            Point2::new(100.0, 300.0),
            100.0,
            2.0,
        )?;

        {
            let _lock = graphics::use_shader(ctx, &self.shader);
            self.shader.send(ctx, self.dim)?;
            graphics::circle(
                ctx,
                graphics::WHITE,
                DrawMode::Fill,
                Point2::new(400.0, 300.0),
                100.0,
                2.0,
            )?;
        }

        graphics::circle(
            ctx,
            graphics::WHITE,
            DrawMode::Fill,
            Point2::new(700.0, 300.0),
            100.0,
            2.0,
        )?;

        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let c = conf::Conf::new();
    let (ctx, events_loop) = &mut Context::load_from_conf("shader", "ggez", c)?;

    // We add the CARGO_MANIFEST_DIR/resources do the filesystems paths so
    // we we look in the cargo project for files.
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        filesystem::mount(ctx, &path, true);
    }

    let state = &mut MainState::new(ctx)?;
    event::run(ctx, events_loop, state)
}

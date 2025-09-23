#![allow(warnings)]

mod editor;
mod engine;
use editor::Editor;
use clap::*;
use winit::event_loop::EventLoop;
use winit::error::EventLoopError;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct EditorArgs {
    /// path to game script
    #[arg(long, default_value_t = String::new())]
    path: String,
    /// Width of Window
    #[arg(long, default_value_t = 600)]
    width: u32,
    /// Height of WIndow
    #[arg(long, default_value_t = 480)]
    height: u32,
    /// Off/On VSync
    #[arg(long, default_value_t = true)]
    pub vsync: bool,
}

fn main() -> Result<(), EventLoopError>{

    let args = EditorArgs::parse();
    let event_loop = EventLoop::new().expect("Error create Event loop");
    let editor = Editor::new(event_loop, args);

    Ok(editor.run()?)
}

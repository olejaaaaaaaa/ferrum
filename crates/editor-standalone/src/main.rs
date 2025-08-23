#![allow(warnings)]

mod editor;
use editor::Editor;
use clap::*;
use winit::event_loop::EventLoop;
use winit::error::EventLoopError;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct EditorArgs {
    /// path to game script
    #[arg(short, long, default_value_t = String::new())]
    path: String,

    #[arg(long, default_value_t = 600)]
    width: u32,

    #[arg(long, default_value_t = 480)]
    height: u32
}

fn main() -> Result<(), EventLoopError>{

    let args = EditorArgs::parse();
    let event_loop = EventLoop::new().expect("Error create Event loop");
    let editor = Editor::new(event_loop, args);

    Ok(editor.run()?)
}


mod texture;
mod graph;
mod node;
mod resource;
mod builder;

use ferrum_render::RenderContext;
use resource::FrameGraphResource;

trait Version {
    fn get_version(&self) -> FrameGraphResource;
}

trait Id {
    fn get_id(&self) -> FrameGraphResource;
}

trait Concept {
    type Desc;
    fn create(desc: Self::Desc);
    fn destroy(desc: Self::Desc);
    fn pre_read(desc: Self::Desc, flags: u32, ctx: &RenderContext);
    fn pre_write(desc: Self::Desc, flags: u32, ctx: &RenderContext);
    fn to_string(desc: Self::Desc) -> String;
}
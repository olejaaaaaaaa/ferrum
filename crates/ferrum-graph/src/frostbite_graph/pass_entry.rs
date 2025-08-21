use std::any::Any;
use crate::frostbite_graph::frame_graph::Execute;

pub struct FrameGraphPassConcept<'n, T> {
    execFunction: &'n Execute<T>,
    data: Box<dyn Any>
}
// ResourceEntry.rs
use std::any::Any;
use std::fmt;
use std::ptr;

use crate::frostbite_graph::pass_node::PassNode;
use crate::frostbite_graph::render_context::RenderContext;
use crate::frostbite_graph::transient_resources::TransientResources;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Type {
    Transient,
    Imported,
}

pub struct ResourceEntry {
    ty: Type,
    id: u32,
    version: u32,
    concept: Box<dyn Concept>,
    producer: *mut PassNode,
    last: *mut PassNode,
}

impl ResourceEntry {
    pub const INITIAL_VERSION: u32 = 1;

    pub fn to_string(&self) -> String {
        self.concept.to_string()
    }

    pub fn create(&mut self, allocator: &TransientResources) {
        assert!(self.is_transient(), "Only transient resources can be created");
        self.concept.create(allocator);
    }

    pub fn destroy(&mut self, allocator: &TransientResources) {
        assert!(self.is_transient(), "Only transient resources can be destroyed");
        self.concept.destroy(allocator);
    }

    pub fn pre_read(&self, flags: u32, context: &RenderContext) {
        self.concept.pre_read(flags, context);
    }

    pub fn pre_write(&self, flags: u32, context: &RenderContext) {
        self.concept.pre_write(flags, context);
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn version(&self) -> u32 {
        self.version
    }

    pub fn is_imported(&self) -> bool {
        self.ty == Type::Imported
    }

    pub fn is_transient(&self) -> bool {
        self.ty == Type::Transient
    }

    pub fn get<T: Resource + 'static>(&self) -> &T {
        &self._get_model::<T>().resource
    }

    pub fn get_mut<T: Resource + 'static>(&mut self) -> &mut T {
        &mut self._get_model_mut::<T>().resource
    }

    pub fn get_descriptor<T: Resource + 'static>(&self) -> &T::Desc {
        &self._get_model::<T>().descriptor
    }

    fn _get_model<T: Resource + 'static>(&self) -> &Model<T> {
        self.concept.as_any().downcast_ref::<Model<T>>().expect("Invalid type")
    }

    fn _get_model_mut<T: Resource + 'static>(&mut self) -> &mut Model<T> {
        self.concept.as_any_mut().downcast_mut::<Model<T>>().expect("Invalid type")
    }

    pub(crate) fn new_with_type<T: Resource + 'static>(
        resource_type: Type,
        id: u32,
        descriptor: T::Desc,
        resource: T,
    ) -> Self {
        Self {
            ty: resource_type,
            id: id,
            version: Self::INITIAL_VERSION,
            concept: Box::new(Model::new(descriptor, resource)),
            producer: ptr::null_mut(),
            last: ptr::null_mut(),
        }
    }
}


trait Concept: Any {
    fn create(&mut self, allocator: &TransientResources);
    fn destroy(&mut self, allocator: &TransientResources);
    fn pre_read(&self, flags: u32, context: &RenderContext);
    fn pre_write(&self, flags: u32, context: &RenderContext);
    fn to_string(&self) -> String;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

struct Model<T: Resource> {
    descriptor: T::Desc,
    resource: T,
}

impl<T: Resource> Model<T> {
    fn new(descriptor: T::Desc, resource: T) -> Self {
        Self { descriptor, resource }
    }
}

impl<T: Resource> Concept for Model<T> {

    fn create(&mut self, allocator: &TransientResources) {
        self.resource.create(&self.descriptor, allocator);
    }

    fn destroy(&mut self, allocator: &TransientResources) {
        self.resource.destroy(&self.descriptor, allocator);
    }

    fn pre_read(&self, flags: u32, ctx: &RenderContext) {
        self.resource.pre_read(&self.descriptor, flags, ctx);
    }

    fn pre_write(&self, flags: u32, ctx: &RenderContext) {
        self.resource.pre_write(&self.descriptor, flags, ctx);
    }

    fn to_string(&self) -> String {
        String::new()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

// Базовый типаж для всех ресурсов
pub trait Resource: 'static {

    type Desc: 'static;

    fn create(&mut self, descriptor: &Self::Desc, allocator: &TransientResources);
    fn destroy(&mut self, descriptor: &Self::Desc, allocator: &TransientResources);
    fn pre_read(&self, descriptor: &Self::Desc, flags: u32, ctx: &RenderContext);
    fn pre_write(&self, descriptor: &Self::Desc, flags: u32, ctx: &RenderContext);
    fn to_string(descriptor: &Self::Desc) -> String;
}



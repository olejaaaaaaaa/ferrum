// ResourceEntry.rs
use std::any::Any;
use std::fmt;
use std::ptr;

use crate::frostbite_graph::pass_node::PassNode;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Type {
    Transient,
    Imported,
}

pub struct ResourceEntry {
    m_type: Type,
    m_id: u32,
    m_version: u32,
    m_concept: Box<dyn Concept>,
    m_producer: *mut PassNode,
    m_last: *mut PassNode,
}

impl ResourceEntry {
    pub const INITIAL_VERSION: u32 = 1;

    pub fn to_string(&self) -> String {
        self.m_concept.to_string()
    }

    pub fn create(&mut self, allocator: &dyn Any) {
        assert!(self.is_transient(), "Only transient resources can be created");
        self.m_concept.create(allocator);
    }

    pub fn destroy(&mut self, allocator: &dyn Any) {
        assert!(self.is_transient(), "Only transient resources can be destroyed");
        self.m_concept.destroy(allocator);
    }

    pub fn pre_read(&self, flags: u32, context: &dyn Any) {
        self.m_concept.pre_read(flags, context);
    }

    pub fn pre_write(&self, flags: u32, context: &dyn Any) {
        self.m_concept.pre_write(flags, context);
    }

    pub fn id(&self) -> u32 {
        self.m_id
    }

    pub fn version(&self) -> u32 {
        self.m_version
    }

    pub fn is_imported(&self) -> bool {
        self.m_type == Type::Imported
    }

    pub fn is_transient(&self) -> bool {
        self.m_type == Type::Transient
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
        self.m_concept.as_any().downcast_ref::<Model<T>>().expect("Invalid type")
    }

    fn _get_model_mut<T: Resource + 'static>(&mut self) -> &mut Model<T> {
        self.m_concept.as_any_mut().downcast_mut::<Model<T>>().expect("Invalid type")
    }

    pub(crate) fn new_with_type<T: Resource + 'static>(
        resource_type: Type,
        id: u32,
        descriptor: T::Desc,
        resource: T,
    ) -> Self {
        Self {
            m_type: resource_type,
            m_id: id,
            m_version: Self::INITIAL_VERSION,
            m_concept: Box::new(Model::new(descriptor, resource)),
            m_producer: ptr::null_mut(),
            m_last: ptr::null_mut(),
        }
    }
}

// Concept trait (аналог C++ abstract class)
trait Concept: Any {
    fn create(&mut self, allocator: &dyn Any);
    fn destroy(&mut self, allocator: &dyn Any);
    fn pre_read(&self, flags: u32, context: &dyn Any);
    fn pre_write(&self, flags: u32, context: &dyn Any);
    fn to_string(&self) -> String;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

// Model struct (аналог C++ template class Model<T>)
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

    fn create(&mut self, allocator: &dyn Any) {
        self.resource.create(&self.descriptor, allocator);
    }

    fn destroy(&mut self, allocator: &dyn Any) {
        self.resource.destroy(&self.descriptor, allocator);
    }

    fn pre_read(&self, flags: u32, ctx: &dyn Any) {
        if has_pre_read::<T>() {
            self.resource.pre_read(&self.descriptor, flags, ctx);
        }
    }

    fn pre_write(&self, flags: u32, ctx: &dyn Any) {
        if has_pre_write::<T>() {
            self.resource.pre_write(&self.descriptor, flags, ctx);
        }
    }

    fn to_string(&self) -> String {
        if has_to_string::<T>() {
            T::to_string(&self.descriptor)
        } else {
            String::new()
        }
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

    fn create(&mut self, descriptor: &Self::Desc, allocator: &dyn Any);
    fn destroy(&mut self, descriptor: &Self::Desc, allocator: &dyn Any);
    fn pre_read(&self, descriptor: &Self::Desc, flags: u32, ctx: &dyn Any);
    fn pre_write(&self, descriptor: &Self::Desc, flags: u32, ctx: &dyn Any);
    fn to_string(descriptor: &Self::Desc) -> String;
}

// Type traits проверки (аналог C++ SFINAE)
fn has_pre_read<T: Resource + 'static>() -> bool {
    // Эмуляция проверки наличия метода
    std::any::TypeId::of::<T>() != std::any::TypeId::of::<()>()
}

fn has_pre_write<T: Resource + 'static>() -> bool {
    // Эмуляция проверки наличия метода
    std::any::TypeId::of::<T>() != std::any::TypeId::of::<()>()
}

fn has_to_string<T: Resource + 'static>() -> bool {
    // Эмуляция проверки наличия статического метода
    std::any::TypeId::of::<T>() != std::any::TypeId::of::<()>()
}

// Реализации для удобства
impl fmt::Debug for ResourceEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ResourceEntry")
            .field("id", &self.m_id)
            .field("version", &self.m_version)
            .field("type", &self.m_type)
            .finish()
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Transient => write!(f, "Transient"),
            Type::Imported => write!(f, "Imported"),
        }
    }
}
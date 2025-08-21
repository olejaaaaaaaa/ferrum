use std::{
    any::{Any, TypeId},
    collections::HashMap
};

pub struct BlackBoard {
    m_storage: HashMap<TypeId, Box<dyn Any>>
}

impl BlackBoard {

    pub fn add<T: Any>(&mut self, value: T) -> &mut T {
        let type_id = TypeId::of::<T>();
        self.m_storage.insert(type_id, Box::new(value));
        self.get_mut::<T>()
    }

    pub fn get<T: Any>(&self) -> &T {
        self.try_get().expect("Type not found in blackboard")
    }

    pub fn get_mut<T: Any>(&mut self) -> &mut T {
        self.try_get_mut().expect("Type not found in blackboard")
    }

    pub fn try_get<T: Any>(&self) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        self.m_storage.get(&type_id).and_then(|boxed| boxed.downcast_ref::<T>())
    }

    pub fn try_get_mut<T: Any>(&mut self) -> Option<&mut T> {
        let type_id = TypeId::of::<T>();
        self.m_storage.get_mut(&type_id).and_then(|boxed| boxed.downcast_mut::<T>())
    }

    pub fn has<T: Any>(&self) -> bool {
        let type_id = TypeId::of::<T>();
        self.m_storage.contains_key(&type_id)
    }

}
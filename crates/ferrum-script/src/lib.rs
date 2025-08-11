

pub trait Script {
    extern "C" fn register_script(&self);
    fn update(&self, dt: usize);
    fn init(&self);
}

pub struct P {

}

impl Script for P {

    extern "C" fn register_script(&self) {

    }

    fn update(&self, dt: usize) {

    }

    fn init(&self) {

    }

}

#[derive(Clone)]
pub enum QueueFamilyProperties {
    V1(Vec<ash::vk::QueueFamilyProperties>),
    V2 {
        raw: Vec<ash::vk::QueueFamilyProperties2<'static>>,
        cached: Vec<ash::vk::QueueFamilyProperties>,
    },
    None
}

impl Default for QueueFamilyProperties {
   fn default() -> Self {
       QueueFamilyProperties::None
   }
}

impl std::ops::Deref for QueueFamilyProperties {
    type Target = Vec<ash::vk::QueueFamilyProperties>;

    fn deref(&self) -> &Self::Target {
        match self {
            QueueFamilyProperties::V1(x) => x,
            QueueFamilyProperties::V2{raw: _, cached } => cached,
            QueueFamilyProperties::None => {
                panic!("QueueFamilyProperties is not initialize")
            }
        }
    }
}
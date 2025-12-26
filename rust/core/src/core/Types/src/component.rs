use crate::core::components::ComponentsHandle;
use crate::core::types::src::base::Base;

pub struct Component {
    pub base: Base,
    pub enabled: bool,
}

impl Component {
    pub fn new(components: ComponentsHandle) -> Self {
        Self {
            base: Base::new(components),
            enabled: true,
        }
    }
}

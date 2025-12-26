use crate::core::components::ComponentsHandle;
use crate::core::types::base::Base;

pub struct Component {
    pub base: Base,
    enabled: bool,
}

impl Component {
    pub fn new(components: ComponentsHandle) -> Self {
        Self {
            base: Base::new(components),
            enabled: false,
        }
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

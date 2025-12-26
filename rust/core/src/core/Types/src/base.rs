use crate::core::components::ComponentsHandle;

#[derive(Debug, Clone, Copy, Default)]
pub struct BaseCapabilities {
    pub disposable: bool,
    pub resizeable: bool,
    pub updateable: bool,
    pub hideable: bool,
    pub configurable: bool,
    pub serializable: bool,
}

pub struct Base {
    pub components: ComponentsHandle,
    pub capabilities: BaseCapabilities,
}

impl Base {
    pub fn new(components: ComponentsHandle) -> Self {
        Self {
            components,
            capabilities: BaseCapabilities::default(),
        }
    }

    pub fn is_disposeable(&self) -> bool {
        self.capabilities.disposable
    }

    pub fn is_resizeable(&self) -> bool {
        self.capabilities.resizeable
    }

    pub fn is_updateable(&self) -> bool {
        self.capabilities.updateable
    }

    pub fn is_hideable(&self) -> bool {
        self.capabilities.hideable
    }

    pub fn is_configurable(&self) -> bool {
        self.capabilities.configurable
    }

    pub fn is_serializable(&self) -> bool {
        self.capabilities.serializable
    }
}

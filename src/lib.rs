use reduct_base::plugin::{Plugin, PluginInfo, PluginInfoBuilder};

pub struct TestPlugin {
    info: PluginInfo,

}

impl Plugin for TestPlugin {
    fn info(&self) -> &PluginInfo {
        &self.info
    }
}

impl TestPlugin {
    pub fn new() -> Self {
        Self {
            info: PluginInfoBuilder::default()
                .name("test".to_string())
                .version("0.1.0".to_string())
                .description("A test plugin".to_string())
                .build()
                .unwrap()
        }
    }
}

#[no_mangle]
pub fn get_plugin() -> *mut dyn Plugin {
    // Return a raw pointer to an instance of our plugin
    Box::into_raw(Box::new(TestPlugin::new()))
}

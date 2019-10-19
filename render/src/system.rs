use crate::device::Device;
use crate::device::Features;
use semver;

pub trait System {
    fn get_devices_count(&self) -> usize;

    fn get_device(&self, index: usize) -> &dyn Device;

    fn get_main_device(&self) -> &dyn Device;

    fn get_device_mut(&mut self, index: usize) -> &mut dyn Device;

    fn get_main_device_mut(&mut self) -> &mut dyn Device;
}

pub trait SystemFabric {
    fn create(request: &SystemRequest) -> Box<dyn System>;
}

pub struct SystemRequest {
    pub api_name: String,

    pub min_supported_version: semver::Version,

    pub first_unsupported_version: semver::Version,

    pub required_device_features: Features,

    pub enabled_device_features: Features,

    pub required_extensions: Vec<String>,

    pub required_layers: Vec<String>,

    pub profile_memory: bool,

    pub profile_perfomance: bool,

    pub debug_validation: bool,

    pub need_rendering: bool,

    pub extensions: Vec<String>,

    pub layers: Vec<String>,

    pub application_name: String,

    pub application_version: semver::Version,

    pub engine_name: String,

    pub engine_version: semver::Version,
}

impl SystemRequest {
    pub fn request_vulkan_debug() -> SystemRequest {
        SystemRequest {
            api_name: Self::vulkan_name(),
            min_supported_version: semver::Version::parse("1.0.0").unwrap(),
            first_unsupported_version: semver::Version::parse("2.0.0").unwrap(),
            required_device_features: Features::none(),
            enabled_device_features: Features::all(),
            required_extensions: vec![],
            required_layers: vec![],
            profile_memory: true,
            profile_perfomance: true,
            debug_validation: true,
            need_rendering: true,
            extensions: vec![],
            layers: vec![],
            application_name: "".to_string(),
            application_version: semver::Version::parse("0.1.0").unwrap(),
            engine_name: "".to_string(),
            engine_version: semver::Version::parse("0.1.0").unwrap(),
        }
    }

    pub fn directx_name() -> String {
        String::from("DirectX")
    }

    pub fn opengl_name() -> String {
        String::from("OpenGL")
    }

    pub fn opengles_name() -> String {
        String::from("OpenGLES")
    }

    pub fn webgl_name() -> String {
        String::from("WebGL")
    }

    pub fn vulkan_name() -> String {
        String::from("Vulkan")
    }

    pub fn metal_name() -> String {
        String::from("Metal")
    }
}

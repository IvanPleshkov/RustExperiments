use semver;

pub trait System {
    // fn render_devices_count() -> usize;
    // fn get_render_device(&self) -> &Box<RenderDevice>;
}

pub trait SystemFabric {

    fn create(params: &SystemInitializationParams) -> Box<dyn System>;
}

pub struct NativeApi {

    pub api_name: String,

    pub min_supported_version: semver::Version,

    pub first_unsupported_version: semver::Version,

    // pub required_features: bool,

    // pub required_extensions: bool,
}

pub struct SystemInitializationParams {
    pub apis: Vec<NativeApi>,
    pub require_memory_profiler: bool,
    pub require_perfomance_profiler: bool,
    pub require_debug_validation: bool,
    pub require_rendering: bool,
    pub use_native_api_extensions: bool,
}

impl SystemInitializationParams {
    pub fn default() -> SystemInitializationParams {
        SystemInitializationParams{
            apis: Self::default_render_apis(),
            require_memory_profiler: true,
            require_perfomance_profiler: true,
            require_debug_validation: true,
            require_rendering: true,
            use_native_api_extensions: false,
        }
    }

    fn default_render_apis() -> Vec<NativeApi> {
        vec![
            NativeApi{
                api_name: "Metal".to_string(),
                min_supported_version: semver::Version::parse("1.0.0").unwrap(),
                first_unsupported_version: semver::Version::parse("2.0.0").unwrap() },
            NativeApi{
                api_name: "DirectX".to_string(),
                min_supported_version: semver::Version::parse("12.0.0").unwrap(),
                first_unsupported_version: semver::Version::parse("13.0.0").unwrap() },
            NativeApi{
                api_name: "Vulkan".to_string(),
                min_supported_version: semver::Version::parse("1.0.0").unwrap(),
                first_unsupported_version: semver::Version::parse("2.0.0").unwrap() },
            NativeApi{
                api_name: "DirectX".to_string(),
                min_supported_version: semver::Version::parse("11.0.0").unwrap(),
                first_unsupported_version: semver::Version::parse("12.0.0").unwrap() },
            NativeApi{
                api_name: "OpenGl".to_string(),
                min_supported_version: semver::Version::parse("4.0.0").unwrap(),
                first_unsupported_version: semver::Version::parse("5.0.0").unwrap() },
            NativeApi{
                api_name: "DirectX".to_string(),
                min_supported_version: semver::Version::parse("10.0.0").unwrap(),
                first_unsupported_version: semver::Version::parse("11.0.0").unwrap() },
            NativeApi{
                api_name: "OpenGl".to_string(),
                min_supported_version: semver::Version::parse("3.0.0").unwrap(),
                first_unsupported_version: semver::Version::parse("4.0.0").unwrap() },
            NativeApi{
                api_name: "OpenGlEs".to_string(),
                min_supported_version: semver::Version::parse("3.0.0").unwrap(),
                first_unsupported_version: semver::Version::parse("4.0.0").unwrap() },
            NativeApi{
                api_name: "DirectX".to_string(),
                min_supported_version: semver::Version::parse("9.0.0").unwrap(),
                first_unsupported_version: semver::Version::parse("10.0.0").unwrap() },
            NativeApi{
                api_name: "OpenGl".to_string(),
                min_supported_version: semver::Version::parse("2.0.0").unwrap(),
                first_unsupported_version: semver::Version::parse("3.0.0").unwrap() },
            NativeApi{
                api_name: "OpenGlEs".to_string(),
                min_supported_version: semver::Version::parse("2.0.0").unwrap(),
                first_unsupported_version: semver::Version::parse("3.0.0").unwrap() },
            NativeApi{
                api_name: "Software".to_string(),
                min_supported_version: semver::Version::parse("1.0.0").unwrap(),
                first_unsupported_version: semver::Version::parse("2.0.0").unwrap() },
        ]
    }
}

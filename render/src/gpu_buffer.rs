use crate::gpu_resource::GpuResource;

pub struct GpuBuffer {
    pub gpu_resource: GpuResource,

    pub gpu_view: GpuResource,

    pub info: GpuBufferInfo,
}

pub enum GpuBufferType {
    Index,
    Vertex,
}

pub struct GpuBufferInfo {
    pub name: String,

    pub size: u64,

    pub buffer_type: GpuBufferType,
}

pub struct GpuBufferInfoBuilder {
    pub info: GpuBufferInfo,
}

impl GpuBufferInfo {
    pub fn default() -> GpuBufferInfo {
        GpuBufferInfo {
            name: String::new(),
            size: 0,
            buffer_type: GpuBufferType::Vertex,
        }
    }
}

impl GpuBufferInfoBuilder {
    pub fn new() -> GpuBufferInfoBuilder {
        GpuBufferInfoBuilder {
            info: GpuBufferInfo::default(),
        }
    }

    pub fn build(self) -> GpuBufferInfo {
        self.info
    }
}

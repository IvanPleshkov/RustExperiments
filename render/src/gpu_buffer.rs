use std::sync::Arc;

pub type GpuBufferHandle = u64;

pub type GpuBufferViewHandle = u64;

pub struct GpuBuffer {
    pub name: String,

    pub handle: Option<GpuBufferHandle>,

    pub view_handle: Option<GpuBufferViewHandle>,

    pub info: Option<GpuBufferInfo>,
}

pub struct GpuBufferInfo {
    pub size: u64,

    pub buffer_type: GpuBufferType,
}

pub enum GpuBufferType {
    Index,
    Vertex,
}

pub struct GpuBufferInfoBuilder {
    pub info: GpuBufferInfo,
}

impl GpuBuffer {
    pub fn new(name: &str) -> Arc<GpuBuffer> {
        Arc::new(GpuBuffer{
            name: String::from(name),
            handle: None,
            view_handle: None,
            info: None,
        })
    }

    pub fn is_valid(&self) -> bool {
        self.handle.is_some() && self.view_handle.is_some()
    }
}

impl GpuBufferInfo {
    pub fn default() -> GpuBufferInfo {
        GpuBufferInfo {
            size: 0,
            buffer_type: GpuBufferType::Vertex,
        }
    }

    pub fn builder() -> GpuBufferInfoBuilder {
        GpuBufferInfoBuilder {
            info: GpuBufferInfo::default(),
        }
    }
}

impl GpuBufferInfoBuilder {
    pub fn build(self) -> GpuBufferInfo {
        self.info
    }

    pub fn size<'a>(&'a mut self, size: u64) -> &'a mut Self {
        self.info.size = size;
        self
    }

    pub fn buffer_type<'a>(&'a mut self, buffer_type: GpuBufferType) -> &'a mut Self {
        self.info.buffer_type = buffer_type;
        self
    }
}

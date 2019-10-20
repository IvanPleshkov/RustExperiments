
pub struct GpuBuffer {
    pub handle: u64,

    pub view_handle: u64,

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

impl GpuBuffer {
    pub fn is_valid(&self) -> bool {
        self.handle != 0 && self.view_handle != 0
    }
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

    pub fn name<'a>(&'a mut self, name: String) -> &'a mut Self {
        self.info.name = name;
        self
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

use crate::command_buffer::CommandBuffer;
use std::sync::Arc;

pub struct GpuBuffer {
    pub id: GpuBufferIndex,

    pub info: GpuBufferInfo,
}

pub struct GpuBufferIndex {
    pub id: u64,
}

pub struct GpuBufferInfo {
    pub name: String,

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
    pub fn new(command_buffer: &mut CommandBuffer, info: GpuBufferInfo) -> Arc<GpuBuffer> {
        Arc::new(GpuBuffer {
            id: GpuBufferIndex {
                id: command_buffer.generate_unique_id(),
            },
            info: info,
        })
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

    pub fn name<'a>(&'a mut self, name: &str) -> &'a mut Self {
        self.info.name = String::from(name);
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

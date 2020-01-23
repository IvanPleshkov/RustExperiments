use crate::command_buffer::CommandBuffer;
use crate::gpu_texture_format::GpuTextureFormat;
use std::sync::Arc;

pub struct GpuTexture {
    pub id: GpuTextureIndex,

    pub info: GpuTextureInfo,
}

pub struct GpuTextureIndex {
    pub id: u64,
}

pub struct GpuTextureInfo {
    pub name: String,

    pub width: u64,

    pub height: u64,

    pub depth: u64,

    pub texture_type: GpuTextureType,

    pub format: GpuTextureFormat,

    pub mips_count: u32,

    pub array_size: u32,

    pub samples: u32,

    pub tiling: GpuTextureTiling,

    pub is_transfer_src: bool,

    pub is_transfer_dst: bool,

    pub is_sampled: bool,

    pub is_storage: bool,
}

pub enum GpuTextureType {
    Texture1D,
    Texture2D,
    Texture3D,
    Texture1DArray,
    Texture2DArray,
    Texture3DArray,
    TextureCubemap,
    DepthStencil,
}

pub enum GpuTextureTiling {
    Linear,
    Optimal,
}

pub struct GpuTextureInfoBuilder {
    pub info: GpuTextureInfo,
}

impl GpuTexture {
    pub fn new(command_buffer: &mut CommandBuffer, info: GpuTextureInfo) -> Arc<GpuTexture> {
        Arc::new(GpuTexture {
            id: GpuTextureIndex {
                id: command_buffer.generate_unique_id(),
            },
            info: info,
        })
    }
}

impl GpuTextureInfo {
    pub fn default() -> GpuTextureInfo {
        GpuTextureInfo {
            name: String::new(),
            width: 1,
            height: 1,
            depth: 1,
            texture_type: GpuTextureType::Texture2D,
            format: GpuTextureFormat::R8G8B8A8Unorm,
            mips_count: 1,
            array_size: 1,
            samples: 1,
            tiling: GpuTextureTiling::Optimal,
            is_transfer_src: true,
            is_transfer_dst: true,
            is_sampled: true,
            is_storage: true,
        }
    }

    pub fn builder() -> GpuTextureInfoBuilder {
        GpuTextureInfoBuilder {
            info: GpuTextureInfo::default(),
        }
    }
}

impl GpuTextureInfoBuilder {
    pub fn new() -> GpuTextureInfoBuilder {
        GpuTextureInfoBuilder {
            info: GpuTextureInfo::default(),
        }
    }

    pub fn build(self) -> GpuTextureInfo {
        self.info
    }

    pub fn name<'a>(&'a mut self, name: &str) -> &'a mut Self {
        self.info.name = String::from(name);
        self
    }

    pub fn width<'a>(&'a mut self, width: u64) -> &'a mut Self {
        self.info.width = width;
        self
    }

    pub fn height<'a>(&'a mut self, height: u64) -> &'a mut Self {
        self.info.height = height;
        self
    }

    pub fn depth<'a>(&'a mut self, depth: u64) -> &'a mut Self {
        self.info.depth = depth;
        self
    }

    pub fn texture_type<'a>(&'a mut self, texture_type: GpuTextureType) -> &'a mut Self {
        self.info.texture_type = texture_type;
        self
    }

    pub fn format<'a>(&'a mut self, format: GpuTextureFormat) -> &'a mut Self {
        self.info.format = format;
        self
    }

    pub fn mips_count<'a>(&'a mut self, mips_count: u32) -> &'a mut Self {
        self.info.mips_count = mips_count;
        self
    }

    pub fn array_size<'a>(&'a mut self, array_size: u32) -> &'a mut Self {
        self.info.array_size = array_size;
        self
    }

    pub fn samples<'a>(&'a mut self, samples: u32) -> &'a mut Self {
        self.info.samples = samples;
        self
    }

    pub fn tiling<'a>(&'a mut self, tiling: GpuTextureTiling) -> &'a mut Self {
        self.info.tiling = tiling;
        self
    }

    pub fn is_transfer_src<'a>(&'a mut self, is_transfer_src: bool) -> &'a mut Self {
        self.info.is_transfer_src = is_transfer_src;
        self
    }

    pub fn is_transfer_dst<'a>(&'a mut self, is_transfer_dst: bool) -> &'a mut Self {
        self.info.is_transfer_dst = is_transfer_dst;
        self
    }

    pub fn is_sampled<'a>(&'a mut self, is_sampled: bool) -> &'a mut Self {
        self.info.is_sampled = is_sampled;
        self
    }

    pub fn is_storage<'a>(&'a mut self, is_storage: bool) -> &'a mut Self {
        self.info.is_storage = is_storage;
        self
    }
}

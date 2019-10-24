pub struct GpuTexture {
    pub handle: u64,

    pub view_handle: u64,

    pub info: GpuTextureInfo,
}

pub enum GpuTextureType {
    Texture1D,
    Texture2D,
    Texture3D,
}

pub enum GpuTextureFormat {}

pub struct GpuTextureInfo {
    pub name: String,

    pub size: u64,

    pub buffer_type: GpuTextureType,
}

pub struct GpuTextureInfoBuilder {
    pub info: GpuTextureInfo,
}

impl GpuTexture {
    pub fn is_valid(&self) -> bool {
        self.handle != 0 && self.view_handle != 0
    }
}

impl GpuTextureInfo {
    pub fn default() -> GpuTextureInfo {
        GpuTextureInfo {
            name: String::new(),
            size: 0,
            buffer_type: GpuTextureType::Texture2D,
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

    pub fn name<'a>(&'a mut self, name: String) -> &'a mut Self {
        self.info.name = name;
        self
    }

    pub fn size<'a>(&'a mut self, size: u64) -> &'a mut Self {
        self.info.size = size;
        self
    }

    pub fn buffer_type<'a>(&'a mut self, buffer_type: GpuTextureType) -> &'a mut Self {
        self.info.buffer_type = buffer_type;
        self
    }
}

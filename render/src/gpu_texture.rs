pub struct GpuTexture {
    pub handle: u64,

    pub view_handle: u64,

    pub info: GpuTextureInfo,
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

pub enum GpuTextureFormat {
    Unknown,
    FormatR4G4UnormPack8,
    FormatR4G4B4A4UnormPack16,
    FormatB4G4R4A4UnormPack16,
    FormatR5G6B5UnormPack16,
    FormatB5G6R5UnormPack16,
    FormatR5G5B5A1UnormPack16,
    FormatB5G5R5A1UnormPack16,
    FormatA1R5G5B5UnormPack16,
    FormatR8Unorm,
    FormatR8Snorm,
    FormatR8Uscaled,
    FormatR8Sscaled,
    FormatR8Uint,
    FormatR8Sint,
    FormatR8Srgb,
    FormatR8G8Unorm,
    FormatR8G8Snorm,
    FormatR8G8Uscaled,
    FormatR8G8Sscaled,
    FormatR8G8Uint,
    FormatR8G8Sint,
    FormatR8G8Srgb,
    FormatR8G8B8Unorm,
    FormatR8G8B8Snorm,
    FormatR8G8B8Uscaled,
    FormatR8G8B8Sscaled,
    FormatR8G8B8Uint,
    FormatR8G8B8Sint,
    FormatR8G8B8Srgb,
    FormatB8G8R8Unorm,
    FormatB8G8R8Snorm,
    FormatB8G8R8Uscaled,
    FormatB8G8R8Sscaled,
    FormatB8G8R8Uint,
    FormatB8G8R8Sint,
    FormatB8G8R8Srgb,
    FormatR8G8B8A8Unorm,
    FormatR8G8B8A8Snorm,
    FormatR8G8B8A8Uscaled,
    FormatR8G8B8A8Sscaled,
    FormatR8G8B8A8Uint,
    FormatR8G8B8A8Sint,
    FormatR8G8B8A8Srgb,
    FormatB8G8R8A8Unorm,
    FormatB8G8R8A8Snorm,
    FormatB8G8R8A8Uscaled,
    FormatB8G8R8A8Sscaled,
    FormatB8G8R8A8Uint,
    FormatB8G8R8A8Sint,
    FormatB8G8R8A8Srgb,
    FormatA8B8G8R8UnormPack32,
    FormatA8B8G8R8SnormPack32,
    FormatA8B8G8R8UscaledPack32,
    FormatA8B8G8R8SscaledPack32,
    FormatA8B8G8R8UintPack32,
    FormatA8B8G8R8SintPack32,
    FormatA8B8G8R8SrgbPack32,
    FormatA2R10G10B10UnormPack32,
    FormatA2R10G10B10SnormPack32,
    FormatA2R10G10B10UscaledPack32,
    FormatA2R10G10B10SscaledPack32,
    FormatA2R10G10B10UintPack32,
    FormatA2R10G10B10SintPack32,
    FormatA2B10G10R10UnormPack32,
    FormatA2B10G10R10SnormPack32,
    FormatA2B10G10R10UscaledPack32,
    FormatA2B10G10R10SscaledPack32,
    FormatA2B10G10R10UintPack32,
    FormatA2B10G10R10SintPack32,
    FormatR16Unorm,
    FormatR16Snorm,
    FormatR16Uscaled,
    FormatR16Sscaled,
    FormatR16Uint,
    FormatR16Sint,
    FormatR16Sfloat,
    FormatR16G16Unorm,
    FormatR16G16Snorm,
    FormatR16G16Uscaled,
    FormatR16G16Sscaled,
    FormatR16G16Uint,
    FormatR16G16Sint,
    FormatR16G16Sfloat,
    FormatR16G16B16Unorm,
    FormatR16G16B16Snorm,
    FormatR16G16B16Uscaled,
    FormatR16G16B16Sscaled,
    FormatR16G16B16Uint,
    FormatR16G16B16Sint,
    FormatR16G16B16Sfloat,
    FormatR16G16B16A16Unorm,
    FormatR16G16B16A16Snorm,
    FormatR16G16B16A16Uscaled,
    FormatR16G16B16A16Sscaled,
    FormatR16G16B16A16Uint,
    FormatR16G16B16A16Sint,
    FormatR16G16B16A16Sfloat,
    FormatR32Uint,
    FormatR32Sint,
    FormatR32Sfloat,
    FormatR32G32Uint,
    FormatR32G32Sint,
    FormatR32G32Sfloat,
    FormatR32G32B32Uint,
    FormatR32G32B32Sint,
    FormatR32G32B32Sfloat,
    FormatR32G32B32A32Uint,
    FormatR32G32B32A32Sint,
    FormatR32G32B32A32Sfloat,
    FormatR64Uint,
    FormatR64Sint,
    FormatR64Sfloat,
    FormatR64G64Uint,
    FormatR64G64Sint,
    FormatR64G64Sfloat,
    FormatR64G64B64Uint,
    FormatR64G64B64Sint,
    FormatR64G64B64Sfloat,
    FormatR64G64B64A64Uint,
    FormatR64G64B64A64Sint,
    FormatR64G64B64A64Sfloat,
    FormatB10G11R11UfloatPack32,
    FormatE5B9G9R9UfloatPack32,
    FormatD16Unorm,
    FormatX8D24UnormPack32,
    FormatD32Sfloat,
    FormatS8Uint,
    FormatD16UnormS8Uint,
    FormatD24UnormS8Uint,
    FormatD32SfloatS8Uint,
    FormatBc1RgbUnormBlock,
    FormatBc1RgbSrgbBlock,
    FormatBc1RgbaUnormBlock,
    FormatBc1RgbaSrgbBlock,
    FormatBc2UnormBlock,
    FormatBc2SrgbBlock,
    FormatBc3UnormBlock,
    FormatBc3SrgbBlock,
    FormatBc4UnormBlock,
    FormatBc4SnormBlock,
    FormatBc5UnormBlock,
    FormatBc5SnormBlock,
    FormatBc6hUfloatBlock,
    FormatBc6hSfloatBlock,
    FormatBc7UnormBlock,
    FormatBc7SrgbBlock,
    FormatEtc2R8G8B8UnormBlock,
    FormatEtc2R8G8B8SrgbBlock,
    FormatEtc2R8G8B8A1UnormBlock,
    FormatEtc2R8G8B8A1SrgbBlock,
    FormatEtc2R8G8B8A8UnormBlock,
    FormatEtc2R8G8B8A8SrgbBlock,
    FormatEacR11UnormBlock,
    FormatEacR11SnormBlock,
    FormatEacR11G11UnormBlock,
    FormatEacR11G11SnormBlock,
    FormatAstc4x4UnormBlock,
    FormatAstc4x4SrgbBlock,
    FormatAstc5x4UnormBlock,
    FormatAstc5x4SrgbBlock,
    FormatAstc5x5UnormBlock,
    FormatAstc5x5SrgbBlock,
    FormatAstc6x5UnormBlock,
    FormatAstc6x5SrgbBlock,
    FormatAstc6x6UnormBlock,
    FormatAstc6x6SrgbBlock,
    FormatAstc8x5UnormBlock,
    FormatAstc8x5SrgbBlock,
    FormatAstc8x6UnormBlock,
    FormatAstc8x6SrgbBlock,
    FormatAstc8x8UnormBlock,
    FormatAstc8x8SrgbBlock,
    FormatAstc10x5UnormBlock,
    FormatAstc10x5SrgbBlock,
    FormatAstc10x6UnormBlock,
    FormatAstc10x6SrgbBlock,
    FormatAstc10x8UnormBlock,
    FormatAstc10x8SrgbBlock,
    FormatAstc10x10UnormBlock,
    FormatAstc10x10SrgbBlock,
    FormatAstc12x10UnormBlock,
    FormatAstc12x10SrgbBlock,
    FormatAstc12x12UnormBlock,
    FormatAstc12x12SrgbBlock,
}

pub enum GpuTextureTiling {
    Linear,
    Optimal,
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
            width: 1,
            height: 1,
            depth: 1,
            texture_type: GpuTextureType::Texture2D,
            format: GpuTextureFormat::FormatR8G8B8A8Unorm,
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

    pub fn name<'a>(&'a mut self, name: String) -> &'a mut Self {
        self.info.name = name;
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

use semver;

pub fn semver_to_vk_version(version: &semver::Version) -> u32 {
    ash::vk_make_version!(version.major, version.minor, version.patch)
}

pub fn vk_version_to_semver(version: u32) -> semver::Version {
    let major = ash::vk_version_major!(version);
    let minor = ash::vk_version_minor!(version);
    let patch = ash::vk_version_patch!(version);
    let string_version = format!("{}.{}.{}", major, minor, patch);
    semver::Version::parse(&string_version).unwrap()
}

pub fn texture_format_to_vk(format: render::GpuTextureFormat) -> Option<ash::vk::Format> {
    match format {
        render::GpuTextureFormat::Unknown => None,
        render::GpuTextureFormat::FormatR4G4UnormPack8 => Some(ash::vk::Format::R4G4_UNORM_PACK8),
        render::GpuTextureFormat::FormatR4G4B4A4UnormPack16 => {
            Some(ash::vk::Format::R4G4B4A4_UNORM_PACK16)
        }
        render::GpuTextureFormat::FormatB4G4R4A4UnormPack16 => {
            Some(ash::vk::Format::B4G4R4A4_UNORM_PACK16)
        }
        render::GpuTextureFormat::FormatR5G6B5UnormPack16 => {
            Some(ash::vk::Format::R5G6B5_UNORM_PACK16)
        }
        render::GpuTextureFormat::FormatB5G6R5UnormPack16 => {
            Some(ash::vk::Format::B5G6R5_UNORM_PACK16)
        }
        render::GpuTextureFormat::FormatR5G5B5A1UnormPack16 => {
            Some(ash::vk::Format::R5G5B5A1_UNORM_PACK16)
        }
        render::GpuTextureFormat::FormatB5G5R5A1UnormPack16 => {
            Some(ash::vk::Format::B5G5R5A1_UNORM_PACK16)
        }
        render::GpuTextureFormat::FormatA1R5G5B5UnormPack16 => {
            Some(ash::vk::Format::A1R5G5B5_UNORM_PACK16)
        }
        render::GpuTextureFormat::FormatR8Unorm => Some(ash::vk::Format::R8_UNORM),
        render::GpuTextureFormat::FormatR8Snorm => Some(ash::vk::Format::R8_SNORM),
        render::GpuTextureFormat::FormatR8Uscaled => Some(ash::vk::Format::R8_USCALED),
        render::GpuTextureFormat::FormatR8Sscaled => Some(ash::vk::Format::R8_SSCALED),
        render::GpuTextureFormat::FormatR8Uint => Some(ash::vk::Format::R8_UINT),
        render::GpuTextureFormat::FormatR8Sint => Some(ash::vk::Format::R8_SINT),
        render::GpuTextureFormat::FormatR8Srgb => Some(ash::vk::Format::R8_SRGB),
        render::GpuTextureFormat::FormatR8G8Unorm => Some(ash::vk::Format::R8G8_UNORM),
        render::GpuTextureFormat::FormatR8G8Snorm => Some(ash::vk::Format::R8G8_SNORM),
        render::GpuTextureFormat::FormatR8G8Uscaled => Some(ash::vk::Format::R8G8_USCALED),
        render::GpuTextureFormat::FormatR8G8Sscaled => Some(ash::vk::Format::R8G8_SSCALED),
        render::GpuTextureFormat::FormatR8G8Uint => Some(ash::vk::Format::R8G8_UINT),
        render::GpuTextureFormat::FormatR8G8Sint => Some(ash::vk::Format::R8G8_SINT),
        render::GpuTextureFormat::FormatR8G8Srgb => Some(ash::vk::Format::R8G8_SRGB),
        render::GpuTextureFormat::FormatR8G8B8Unorm => Some(ash::vk::Format::R8G8B8_UNORM),
        render::GpuTextureFormat::FormatR8G8B8Snorm => Some(ash::vk::Format::R8G8B8_SNORM),
        render::GpuTextureFormat::FormatR8G8B8Uscaled => Some(ash::vk::Format::R8G8B8_USCALED),
        render::GpuTextureFormat::FormatR8G8B8Sscaled => Some(ash::vk::Format::R8G8B8_SSCALED),
        render::GpuTextureFormat::FormatR8G8B8Uint => Some(ash::vk::Format::R8G8B8_UINT),
        render::GpuTextureFormat::FormatR8G8B8Sint => Some(ash::vk::Format::R8G8B8_SINT),
        render::GpuTextureFormat::FormatR8G8B8Srgb => Some(ash::vk::Format::R8G8B8_SRGB),
        render::GpuTextureFormat::FormatB8G8R8Unorm => Some(ash::vk::Format::B8G8R8_UNORM),
        render::GpuTextureFormat::FormatB8G8R8Snorm => Some(ash::vk::Format::B8G8R8_SNORM),
        render::GpuTextureFormat::FormatB8G8R8Uscaled => Some(ash::vk::Format::B8G8R8_USCALED),
        render::GpuTextureFormat::FormatB8G8R8Sscaled => Some(ash::vk::Format::B8G8R8_SSCALED),
        render::GpuTextureFormat::FormatB8G8R8Uint => Some(ash::vk::Format::B8G8R8_UINT),
        render::GpuTextureFormat::FormatB8G8R8Sint => Some(ash::vk::Format::B8G8R8_SINT),
        render::GpuTextureFormat::FormatB8G8R8Srgb => Some(ash::vk::Format::B8G8R8_SRGB),
        render::GpuTextureFormat::FormatR8G8B8A8Unorm => Some(ash::vk::Format::R8G8B8A8_UNORM),
        render::GpuTextureFormat::FormatR8G8B8A8Snorm => Some(ash::vk::Format::R8G8B8A8_SNORM),
        render::GpuTextureFormat::FormatR8G8B8A8Uscaled => Some(ash::vk::Format::R8G8B8A8_USCALED),
        render::GpuTextureFormat::FormatR8G8B8A8Sscaled => Some(ash::vk::Format::R8G8B8A8_SSCALED),
        render::GpuTextureFormat::FormatR8G8B8A8Uint => Some(ash::vk::Format::R8G8B8A8_UINT),
        render::GpuTextureFormat::FormatR8G8B8A8Sint => Some(ash::vk::Format::R8G8B8A8_SINT),
        render::GpuTextureFormat::FormatR8G8B8A8Srgb => Some(ash::vk::Format::R8G8B8A8_SRGB),
        render::GpuTextureFormat::FormatB8G8R8A8Unorm => Some(ash::vk::Format::B8G8R8A8_UNORM),
        render::GpuTextureFormat::FormatB8G8R8A8Snorm => Some(ash::vk::Format::B8G8R8A8_SNORM),
        render::GpuTextureFormat::FormatB8G8R8A8Uscaled => Some(ash::vk::Format::B8G8R8A8_USCALED),
        render::GpuTextureFormat::FormatB8G8R8A8Sscaled => Some(ash::vk::Format::B8G8R8A8_SSCALED),
        render::GpuTextureFormat::FormatB8G8R8A8Uint => Some(ash::vk::Format::B8G8R8A8_UINT),
        render::GpuTextureFormat::FormatB8G8R8A8Sint => Some(ash::vk::Format::B8G8R8A8_SINT),
        render::GpuTextureFormat::FormatB8G8R8A8Srgb => Some(ash::vk::Format::B8G8R8A8_SRGB),
        render::GpuTextureFormat::FormatA8B8G8R8UnormPack32 => {
            Some(ash::vk::Format::A8B8G8R8_UNORM_PACK32)
        }
        render::GpuTextureFormat::FormatA8B8G8R8SnormPack32 => {
            Some(ash::vk::Format::A8B8G8R8_SNORM_PACK32)
        }
        render::GpuTextureFormat::FormatA8B8G8R8UscaledPack32 => {
            Some(ash::vk::Format::A8B8G8R8_USCALED_PACK32)
        }
        render::GpuTextureFormat::FormatA8B8G8R8SscaledPack32 => {
            Some(ash::vk::Format::A8B8G8R8_SSCALED_PACK32)
        }
        render::GpuTextureFormat::FormatA8B8G8R8UintPack32 => {
            Some(ash::vk::Format::A8B8G8R8_UINT_PACK32)
        }
        render::GpuTextureFormat::FormatA8B8G8R8SintPack32 => {
            Some(ash::vk::Format::A8B8G8R8_SINT_PACK32)
        }
        render::GpuTextureFormat::FormatA8B8G8R8SrgbPack32 => {
            Some(ash::vk::Format::A8B8G8R8_SRGB_PACK32)
        }
        render::GpuTextureFormat::FormatA2R10G10B10UnormPack32 => {
            Some(ash::vk::Format::A2R10G10B10_UNORM_PACK32)
        }
        render::GpuTextureFormat::FormatA2R10G10B10SnormPack32 => {
            Some(ash::vk::Format::A2R10G10B10_SNORM_PACK32)
        }
        render::GpuTextureFormat::FormatA2R10G10B10UscaledPack32 => {
            Some(ash::vk::Format::A2R10G10B10_USCALED_PACK32)
        }
        render::GpuTextureFormat::FormatA2R10G10B10SscaledPack32 => {
            Some(ash::vk::Format::A2R10G10B10_SSCALED_PACK32)
        }
        render::GpuTextureFormat::FormatA2R10G10B10UintPack32 => {
            Some(ash::vk::Format::A2R10G10B10_UINT_PACK32)
        }
        render::GpuTextureFormat::FormatA2R10G10B10SintPack32 => {
            Some(ash::vk::Format::A2R10G10B10_SINT_PACK32)
        }
        render::GpuTextureFormat::FormatA2B10G10R10UnormPack32 => {
            Some(ash::vk::Format::A2B10G10R10_UNORM_PACK32)
        }
        render::GpuTextureFormat::FormatA2B10G10R10SnormPack32 => {
            Some(ash::vk::Format::A2B10G10R10_SNORM_PACK32)
        }
        render::GpuTextureFormat::FormatA2B10G10R10UscaledPack32 => {
            Some(ash::vk::Format::A2B10G10R10_USCALED_PACK32)
        }
        render::GpuTextureFormat::FormatA2B10G10R10SscaledPack32 => {
            Some(ash::vk::Format::A2B10G10R10_SSCALED_PACK32)
        }
        render::GpuTextureFormat::FormatA2B10G10R10UintPack32 => {
            Some(ash::vk::Format::A2B10G10R10_UINT_PACK32)
        }
        render::GpuTextureFormat::FormatA2B10G10R10SintPack32 => {
            Some(ash::vk::Format::A2B10G10R10_SINT_PACK32)
        }
        render::GpuTextureFormat::FormatR16Unorm => Some(ash::vk::Format::R16_UNORM),
        render::GpuTextureFormat::FormatR16Snorm => Some(ash::vk::Format::R16_SNORM),
        render::GpuTextureFormat::FormatR16Uscaled => Some(ash::vk::Format::R16_USCALED),
        render::GpuTextureFormat::FormatR16Sscaled => Some(ash::vk::Format::R16_SSCALED),
        render::GpuTextureFormat::FormatR16Uint => Some(ash::vk::Format::R16_UINT),
        render::GpuTextureFormat::FormatR16Sint => Some(ash::vk::Format::R16_SINT),
        render::GpuTextureFormat::FormatR16Sfloat => Some(ash::vk::Format::R16_SFLOAT),
        render::GpuTextureFormat::FormatR16G16Unorm => Some(ash::vk::Format::R16G16_UNORM),
        render::GpuTextureFormat::FormatR16G16Snorm => Some(ash::vk::Format::R16G16_SNORM),
        render::GpuTextureFormat::FormatR16G16Uscaled => Some(ash::vk::Format::R16G16_USCALED),
        render::GpuTextureFormat::FormatR16G16Sscaled => Some(ash::vk::Format::R16G16_SSCALED),
        render::GpuTextureFormat::FormatR16G16Uint => Some(ash::vk::Format::R16G16_UINT),
        render::GpuTextureFormat::FormatR16G16Sint => Some(ash::vk::Format::R16G16_SINT),
        render::GpuTextureFormat::FormatR16G16Sfloat => Some(ash::vk::Format::R16G16_SFLOAT),
        render::GpuTextureFormat::FormatR16G16B16Unorm => Some(ash::vk::Format::R16G16B16_UNORM),
        render::GpuTextureFormat::FormatR16G16B16Snorm => Some(ash::vk::Format::R16G16B16_SNORM),
        render::GpuTextureFormat::FormatR16G16B16Uscaled => {
            Some(ash::vk::Format::R16G16B16_USCALED)
        }
        render::GpuTextureFormat::FormatR16G16B16Sscaled => {
            Some(ash::vk::Format::R16G16B16_SSCALED)
        }
        render::GpuTextureFormat::FormatR16G16B16Uint => Some(ash::vk::Format::R16G16B16_UINT),
        render::GpuTextureFormat::FormatR16G16B16Sint => Some(ash::vk::Format::R16G16B16_SINT),
        render::GpuTextureFormat::FormatR16G16B16Sfloat => Some(ash::vk::Format::R16G16B16_SFLOAT),
        render::GpuTextureFormat::FormatR16G16B16A16Unorm => {
            Some(ash::vk::Format::R16G16B16A16_UNORM)
        }
        render::GpuTextureFormat::FormatR16G16B16A16Snorm => {
            Some(ash::vk::Format::R16G16B16A16_SNORM)
        }
        render::GpuTextureFormat::FormatR16G16B16A16Uscaled => {
            Some(ash::vk::Format::R16G16B16A16_USCALED)
        }
        render::GpuTextureFormat::FormatR16G16B16A16Sscaled => {
            Some(ash::vk::Format::R16G16B16A16_SSCALED)
        }
        render::GpuTextureFormat::FormatR16G16B16A16Uint => {
            Some(ash::vk::Format::R16G16B16A16_UINT)
        }
        render::GpuTextureFormat::FormatR16G16B16A16Sint => {
            Some(ash::vk::Format::R16G16B16A16_SINT)
        }
        render::GpuTextureFormat::FormatR16G16B16A16Sfloat => {
            Some(ash::vk::Format::R16G16B16A16_SFLOAT)
        }
        render::GpuTextureFormat::FormatR32Uint => Some(ash::vk::Format::R32_UINT),
        render::GpuTextureFormat::FormatR32Sint => Some(ash::vk::Format::R32_SINT),
        render::GpuTextureFormat::FormatR32Sfloat => Some(ash::vk::Format::R32_SFLOAT),
        render::GpuTextureFormat::FormatR32G32Uint => Some(ash::vk::Format::R32G32_UINT),
        render::GpuTextureFormat::FormatR32G32Sint => Some(ash::vk::Format::R32G32_SINT),
        render::GpuTextureFormat::FormatR32G32Sfloat => Some(ash::vk::Format::R32G32_SFLOAT),
        render::GpuTextureFormat::FormatR32G32B32Uint => Some(ash::vk::Format::R32G32B32_UINT),
        render::GpuTextureFormat::FormatR32G32B32Sint => Some(ash::vk::Format::R32G32B32_SINT),
        render::GpuTextureFormat::FormatR32G32B32Sfloat => Some(ash::vk::Format::R32G32B32_SFLOAT),
        render::GpuTextureFormat::FormatR32G32B32A32Uint => {
            Some(ash::vk::Format::R32G32B32A32_UINT)
        }
        render::GpuTextureFormat::FormatR32G32B32A32Sint => {
            Some(ash::vk::Format::R32G32B32A32_SINT)
        }
        render::GpuTextureFormat::FormatR32G32B32A32Sfloat => {
            Some(ash::vk::Format::R32G32B32A32_SFLOAT)
        }
        render::GpuTextureFormat::FormatR64Uint => Some(ash::vk::Format::R64_UINT),
        render::GpuTextureFormat::FormatR64Sint => Some(ash::vk::Format::R64_SINT),
        render::GpuTextureFormat::FormatR64Sfloat => Some(ash::vk::Format::R64_SFLOAT),
        render::GpuTextureFormat::FormatR64G64Uint => Some(ash::vk::Format::R64G64_UINT),
        render::GpuTextureFormat::FormatR64G64Sint => Some(ash::vk::Format::R64G64_SINT),
        render::GpuTextureFormat::FormatR64G64Sfloat => Some(ash::vk::Format::R64G64_SFLOAT),
        render::GpuTextureFormat::FormatR64G64B64Uint => Some(ash::vk::Format::R64G64B64_UINT),
        render::GpuTextureFormat::FormatR64G64B64Sint => Some(ash::vk::Format::R64G64B64_SINT),
        render::GpuTextureFormat::FormatR64G64B64Sfloat => Some(ash::vk::Format::R64G64B64_SFLOAT),
        render::GpuTextureFormat::FormatR64G64B64A64Uint => {
            Some(ash::vk::Format::R64G64B64A64_UINT)
        }
        render::GpuTextureFormat::FormatR64G64B64A64Sint => {
            Some(ash::vk::Format::R64G64B64A64_SINT)
        }
        render::GpuTextureFormat::FormatR64G64B64A64Sfloat => {
            Some(ash::vk::Format::R64G64B64A64_SFLOAT)
        }
        render::GpuTextureFormat::FormatB10G11R11UfloatPack32 => {
            Some(ash::vk::Format::B10G11R11_UFLOAT_PACK32)
        }
        render::GpuTextureFormat::FormatE5B9G9R9UfloatPack32 => {
            Some(ash::vk::Format::E5B9G9R9_UFLOAT_PACK32)
        }
        render::GpuTextureFormat::FormatD16Unorm => Some(ash::vk::Format::D16_UNORM),
        render::GpuTextureFormat::FormatX8D24UnormPack32 => {
            Some(ash::vk::Format::X8_D24_UNORM_PACK32)
        }
        render::GpuTextureFormat::FormatD32Sfloat => Some(ash::vk::Format::D32_SFLOAT),
        render::GpuTextureFormat::FormatS8Uint => Some(ash::vk::Format::S8_UINT),
        render::GpuTextureFormat::FormatD16UnormS8Uint => Some(ash::vk::Format::D16_UNORM_S8_UINT),
        render::GpuTextureFormat::FormatD24UnormS8Uint => Some(ash::vk::Format::D24_UNORM_S8_UINT),
        render::GpuTextureFormat::FormatD32SfloatS8Uint => {
            Some(ash::vk::Format::D32_SFLOAT_S8_UINT)
        }
        render::GpuTextureFormat::FormatBc1RgbUnormBlock => {
            Some(ash::vk::Format::BC1_RGB_UNORM_BLOCK)
        }
        render::GpuTextureFormat::FormatBc1RgbSrgbBlock => {
            Some(ash::vk::Format::BC1_RGB_SRGB_BLOCK)
        }
        render::GpuTextureFormat::FormatBc1RgbaUnormBlock => {
            Some(ash::vk::Format::BC1_RGBA_UNORM_BLOCK)
        }
        render::GpuTextureFormat::FormatBc1RgbaSrgbBlock => {
            Some(ash::vk::Format::BC1_RGBA_SRGB_BLOCK)
        }
        render::GpuTextureFormat::FormatBc2UnormBlock => Some(ash::vk::Format::BC2_UNORM_BLOCK),
        render::GpuTextureFormat::FormatBc2SrgbBlock => Some(ash::vk::Format::BC2_SRGB_BLOCK),
        render::GpuTextureFormat::FormatBc3UnormBlock => Some(ash::vk::Format::BC3_UNORM_BLOCK),
        render::GpuTextureFormat::FormatBc3SrgbBlock => Some(ash::vk::Format::BC3_SRGB_BLOCK),
        render::GpuTextureFormat::FormatBc4UnormBlock => Some(ash::vk::Format::BC4_UNORM_BLOCK),
        render::GpuTextureFormat::FormatBc4SnormBlock => Some(ash::vk::Format::BC4_SNORM_BLOCK),
        render::GpuTextureFormat::FormatBc5UnormBlock => Some(ash::vk::Format::BC5_UNORM_BLOCK),
        render::GpuTextureFormat::FormatBc5SnormBlock => Some(ash::vk::Format::BC5_SNORM_BLOCK),
        render::GpuTextureFormat::FormatBc6hUfloatBlock => Some(ash::vk::Format::BC6H_UFLOAT_BLOCK),
        render::GpuTextureFormat::FormatBc6hSfloatBlock => Some(ash::vk::Format::BC6H_SFLOAT_BLOCK),
        render::GpuTextureFormat::FormatBc7UnormBlock => Some(ash::vk::Format::BC7_UNORM_BLOCK),
        render::GpuTextureFormat::FormatBc7SrgbBlock => Some(ash::vk::Format::BC7_SRGB_BLOCK),
        render::GpuTextureFormat::FormatEtc2R8G8B8UnormBlock => {
            Some(ash::vk::Format::ETC2_R8G8B8_UNORM_BLOCK)
        }
        render::GpuTextureFormat::FormatEtc2R8G8B8SrgbBlock => {
            Some(ash::vk::Format::ETC2_R8G8B8_SRGB_BLOCK)
        }
        render::GpuTextureFormat::FormatEtc2R8G8B8A1UnormBlock => {
            Some(ash::vk::Format::ETC2_R8G8B8A1_UNORM_BLOCK)
        }
        render::GpuTextureFormat::FormatEtc2R8G8B8A1SrgbBlock => {
            Some(ash::vk::Format::ETC2_R8G8B8A1_SRGB_BLOCK)
        }
        render::GpuTextureFormat::FormatEtc2R8G8B8A8UnormBlock => {
            Some(ash::vk::Format::ETC2_R8G8B8A8_UNORM_BLOCK)
        }
        render::GpuTextureFormat::FormatEtc2R8G8B8A8SrgbBlock => {
            Some(ash::vk::Format::ETC2_R8G8B8A8_SRGB_BLOCK)
        }
        render::GpuTextureFormat::FormatEacR11UnormBlock => {
            Some(ash::vk::Format::EAC_R11_UNORM_BLOCK)
        }
        render::GpuTextureFormat::FormatEacR11SnormBlock => {
            Some(ash::vk::Format::EAC_R11_SNORM_BLOCK)
        }
        render::GpuTextureFormat::FormatEacR11G11UnormBlock => {
            Some(ash::vk::Format::EAC_R11G11_UNORM_BLOCK)
        }
        render::GpuTextureFormat::FormatEacR11G11SnormBlock => {
            Some(ash::vk::Format::EAC_R11G11_SNORM_BLOCK)
        }
        render::GpuTextureFormat::FormatAstc4x4UnormBlock => {
            Some(ash::vk::Format::ASTC_4X4_UNORM_BLOCK)
        }
        render::GpuTextureFormat::FormatAstc4x4SrgbBlock => {
            Some(ash::vk::Format::ASTC_4X4_SRGB_BLOCK)
        }
        render::GpuTextureFormat::FormatAstc5x4UnormBlock => {
            Some(ash::vk::Format::ASTC_5X4_UNORM_BLOCK)
        }
        render::GpuTextureFormat::FormatAstc5x4SrgbBlock => {
            Some(ash::vk::Format::ASTC_5X4_SRGB_BLOCK)
        }
        render::GpuTextureFormat::FormatAstc5x5UnormBlock => {
            Some(ash::vk::Format::ASTC_5X5_UNORM_BLOCK)
        }
        render::GpuTextureFormat::FormatAstc5x5SrgbBlock => {
            Some(ash::vk::Format::ASTC_5X5_SRGB_BLOCK)
        }
        render::GpuTextureFormat::FormatAstc6x5UnormBlock => {
            Some(ash::vk::Format::ASTC_6X5_UNORM_BLOCK)
        }
        render::GpuTextureFormat::FormatAstc6x5SrgbBlock => {
            Some(ash::vk::Format::ASTC_6X5_SRGB_BLOCK)
        }
        render::GpuTextureFormat::FormatAstc6x6UnormBlock => {
            Some(ash::vk::Format::ASTC_6X6_UNORM_BLOCK)
        }
        render::GpuTextureFormat::FormatAstc6x6SrgbBlock => {
            Some(ash::vk::Format::ASTC_6X6_SRGB_BLOCK)
        }
        render::GpuTextureFormat::FormatAstc8x5UnormBlock => {
            Some(ash::vk::Format::ASTC_8X5_UNORM_BLOCK)
        }
        render::GpuTextureFormat::FormatAstc8x5SrgbBlock => {
            Some(ash::vk::Format::ASTC_8X5_SRGB_BLOCK)
        }
        render::GpuTextureFormat::FormatAstc8x6UnormBlock => {
            Some(ash::vk::Format::ASTC_8X6_UNORM_BLOCK)
        }
        render::GpuTextureFormat::FormatAstc8x6SrgbBlock => {
            Some(ash::vk::Format::ASTC_8X6_SRGB_BLOCK)
        }
        render::GpuTextureFormat::FormatAstc8x8UnormBlock => {
            Some(ash::vk::Format::ASTC_8X8_UNORM_BLOCK)
        }
        render::GpuTextureFormat::FormatAstc8x8SrgbBlock => {
            Some(ash::vk::Format::ASTC_8X8_SRGB_BLOCK)
        }
        render::GpuTextureFormat::FormatAstc10x5UnormBlock => {
            Some(ash::vk::Format::ASTC_10X5_UNORM_BLOCK)
        }
        render::GpuTextureFormat::FormatAstc10x5SrgbBlock => {
            Some(ash::vk::Format::ASTC_10X5_SRGB_BLOCK)
        }
        render::GpuTextureFormat::FormatAstc10x6UnormBlock => {
            Some(ash::vk::Format::ASTC_10X6_UNORM_BLOCK)
        }
        render::GpuTextureFormat::FormatAstc10x6SrgbBlock => {
            Some(ash::vk::Format::ASTC_10X6_SRGB_BLOCK)
        }
        render::GpuTextureFormat::FormatAstc10x8UnormBlock => {
            Some(ash::vk::Format::ASTC_10X8_UNORM_BLOCK)
        }
        render::GpuTextureFormat::FormatAstc10x8SrgbBlock => {
            Some(ash::vk::Format::ASTC_10X8_SRGB_BLOCK)
        }
        render::GpuTextureFormat::FormatAstc10x10UnormBlock => {
            Some(ash::vk::Format::ASTC_10X10_UNORM_BLOCK)
        }
        render::GpuTextureFormat::FormatAstc10x10SrgbBlock => {
            Some(ash::vk::Format::ASTC_10X10_SRGB_BLOCK)
        }
        render::GpuTextureFormat::FormatAstc12x10UnormBlock => {
            Some(ash::vk::Format::ASTC_12X10_UNORM_BLOCK)
        }
        render::GpuTextureFormat::FormatAstc12x10SrgbBlock => {
            Some(ash::vk::Format::ASTC_12X10_SRGB_BLOCK)
        }
        render::GpuTextureFormat::FormatAstc12x12UnormBlock => {
            Some(ash::vk::Format::ASTC_12X12_UNORM_BLOCK)
        }
        render::GpuTextureFormat::FormatAstc12x12SrgbBlock => {
            Some(ash::vk::Format::ASTC_12X12_SRGB_BLOCK)
        }
    }
}

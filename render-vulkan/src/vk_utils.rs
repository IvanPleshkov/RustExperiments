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

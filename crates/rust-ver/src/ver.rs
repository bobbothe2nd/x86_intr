//! Lightweight version management built off `rustc --version --verbose`

use std::{
    fmt::{self, Display, Formatter}, process::Command, str::{FromStr, from_utf8}, sync::LazyLock,
};

use crate::Error;

/// Trait to parse generic types to versions
pub trait ToVersion {
    /// Convert generic types to a version by parsing/forwarding
    fn to_version(self) -> Result<Version, Error>;
}

impl ToVersion for Version {
    #[inline]
    fn to_version(self) -> Result<Version, Error> {
        Ok(self)
    }
}

impl<T: TryInto<Version, Error = Error>> ToVersion for T {
    #[inline]
    fn to_version(self) -> Result<Version, Error> {
        self.try_into()
    }
}

impl TryFrom<&str> for Version {
    type Error = Error;

    #[inline]
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl TryFrom<&String> for Version {
    type Error = Error;

    #[inline]
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl TryFrom<String> for Version {
    type Error = Error;

    #[inline]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

/// Represents a basic version with only 3 segments and no prerelease or build metadata
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Version {
    /// 1st segment of semver
    pub major: u32,

    /// 2nd segment of semver
    pub minor: u32,

    /// 3rd segment of semver
    pub patch: u32,
}

impl FromStr for Version {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut segs = s.split('.');

        let major = segs.next().ok_or(Error::UnsupportedVersion)?;
        let minor = segs.next().ok_or(Error::UnsupportedVersion)?;
        let patch = segs.next();

        if segs.next().is_some() {
            return Err(Error::UnsupportedVersion);
        }

        let major = major.parse().map_err(|_| Error::UnsupportedVersion)?;
        let minor = minor.parse().map_err(|_| Error::UnsupportedVersion)?;

        let patch = if let Some(patch) = patch {
            patch.parse().map_err(|_| Error::UnsupportedVersion)?
        } else {
            0
        };

        Ok(Version {
            major,
            minor,
            patch,
        })
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl Version {
    /// Structures a version containing major, minor, and patch
    #[inline]
    #[must_use]
    pub const fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }
}

/// Release channel of the compiler.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Channel {
    /// Development release channel (similar to nightly)
    Dev,

    /// Nightly release channel
    Nightly,

    /// Beta release channel
    Beta,

    /// Stable release channel
    Stable,
}

impl FromStr for Channel {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "stable" => Ok(Channel::Stable),
            "beta" => Ok(Channel::Beta),
            "nightly" => Ok(Channel::Nightly),
            "dev" => Ok(Channel::Dev),
            _ => Err(Error::InvalidOutput),
        }
    }
}

impl Display for Channel {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Stable => f.write_str("stable"),
            Self::Beta => f.write_str("beta"),
            Self::Dev => f.write_str("dev"),
            Self::Nightly => f.write_str("nightly"),
        }
    }
}

impl Channel {
    /// Emits a static string suffix that can be directly appended to a version. (e.g. `-beta`)
    pub const fn suffix(self) -> &'static str {
        match self {
            Self::Stable => "",
            Self::Beta => "-beta",
            Self::Dev => "-dev",
            Self::Nightly => "-nightly",
        }
    }
}

static RUSTC_VER: LazyLock<Option<Box<str>>> = LazyLock::new(|| {
    let output = Command::new("rustc").arg("-vV").output().ok()?;

    if !output.status.success() {
        return None;
    }

    Some(Box::from(from_utf8(&output.stdout).ok()?))
});

static RUST_CHANNEL: LazyLock<Result<Channel, Error>> = LazyLock::new(|| {
    let stdout = RUSTC_VER.as_ref().ok_or(Error::RustcFailed)?;

    let version = stdout
        .lines()
        .find(|line| line.starts_with("release: "))
        .ok_or(Error::InvalidOutput)?;
    let Some(channel) = version.split('-').nth(1) else {
        return Ok(Channel::Stable);
    };

    Channel::from_str(channel)
});

static RUST_VERSION: LazyLock<Result<Version, Error>> = LazyLock::new(|| {
    let stdout = RUSTC_VER.as_ref().ok_or(Error::RustcFailed)?;

    let version = stdout
        .lines()
        .find_map(|line| line.strip_prefix("release: "))
        .ok_or(Error::InvalidOutput)?;
    let strip_channel = version.split('-').next().ok_or(Error::InvalidOutput)?;

    Version::from_str(strip_channel)
});

static LLVM_VERSION: LazyLock<Result<Version, Error>> = LazyLock::new(|| {
    let stdout = RUSTC_VER.as_ref().ok_or(Error::RustcFailed)?;

    let version = stdout
        .lines()
        .find_map(|line| line.strip_prefix("LLVM version: "))
        .ok_or(Error::InvalidOutput)?;

    Version::from_str(version)
});

/// Detects current Rust channel used by `rustc`
#[inline]
pub fn rust_channel() -> Result<Channel, Error> {
    *RUST_CHANNEL
}

/// Detects current Rust version used by `rustc`
#[inline]
pub fn rust_ver() -> Result<Version, Error> {
    *RUST_VERSION
}

/// Detects current LLVM version used by `rustc`
#[inline]
pub fn llvm_ver() -> Result<Version, Error> {
    *LLVM_VERSION
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_three_segments() {
        assert_eq!(
            Version::from_str("23.1.1").unwrap(),
            Version {
                major: 23,
                minor: 1,
                patch: 1,
            }
        );
    }

    #[test]
    fn defaults_missing_patch_to_zero() {
        assert_eq!(
            Version::from_str("23.1").unwrap(),
            Version {
                major: 23,
                minor: 1,
                patch: 0,
            }
        );
    }

    #[test]
    fn parses_zero_version() {
        assert_eq!(
            Version::from_str("0.0.0").unwrap(),
            Version {
                major: 0,
                minor: 0,
                patch: 0,
            }
        );
    }

    #[test]
    fn rejects_empty_input() {
        assert_eq!(Version::from_str(""), Err(Error::UnsupportedVersion));
    }

    #[test]
    fn rejects_missing_minor() {
        assert_eq!(Version::from_str("23"), Err(Error::UnsupportedVersion));
    }

    #[test]
    fn rejects_empty_segments() {
        for input in ["23..1", ".1.1", "23.1.", "23."] {
            assert_eq!(
                Version::from_str(input),
                Err(Error::UnsupportedVersion),
                "input: {input:?}"
            );
        }
    }

    #[test]
    fn rejects_extra_segments() {
        for input in ["23.1.1.4", "23.1.1.0.0"] {
            assert_eq!(
                Version::from_str(input),
                Err(Error::UnsupportedVersion),
                "input: {input:?}"
            );
        }
    }

    #[test]
    fn rejects_non_numeric_segments() {
        for input in ["x.1.1", "23.x.1", "23.1.x", "23.1.1-alpha.32", ">=24.8.1"] {
            assert_eq!(
                Version::from_str(input),
                Err(Error::UnsupportedVersion),
                "input: {input:?}"
            );
        }
    }

    #[test]
    fn rejects_numeric_overflow() {
        assert_eq!(
            Version::from_str("4294967296.1.1"),
            Err(Error::UnsupportedVersion)
        );
    }

    #[test]
    fn compares_versions_lexicographically() {
        let v1 = Version::from_str("23.1.1").unwrap();
        let v2 = Version::from_str("23.2.0").unwrap();
        let v3 = Version::from_str("24.0.0").unwrap();

        assert!(v1 < v2);
        assert!(v2 < v3);
        assert!(v1 < v3);
    }

    #[test]
    fn omitted_patch_equals_explicit_zero() {
        assert_eq!(
            Version::from_str("23.1").unwrap(),
            Version::from_str("23.1.0").unwrap()
        );
    }
}

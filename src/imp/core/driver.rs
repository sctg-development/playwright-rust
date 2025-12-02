use crate::imp::prelude::*;
use std::{env, fs, io};
use zip::{result::ZipError, ZipArchive};

#[derive(Debug, Clone, PartialEq)]
pub struct Driver {
    path: PathBuf,
}

impl Driver {
    const ZIP: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), env!("SEP"), "driver.zip"));
    const PLATFORM: &'static str = include_str!(concat!(env!("OUT_DIR"), env!("SEP"), "platform"));

    pub fn install() -> io::Result<Self> {
        let this = Self::new(Self::default_dest());
        if !this.path.is_dir() {
            this.prepare()?;
        }
        Ok(this)
    }

    /// Without prepare
    pub fn new<P: Into<PathBuf>>(path: P) -> Self {
        Self { path: path.into() }
    }
    ///
    pub fn prepare(&self) -> Result<(), ZipError> {
        fs::create_dir_all(&self.path)?;
        let mut a = ZipArchive::new(io::Cursor::new(Self::ZIP))?;
        a.extract(&self.path)
    }

    pub fn default_dest() -> PathBuf {
        let base: PathBuf = dirs::cache_dir().unwrap_or_else(env::temp_dir);
        let dir: PathBuf = [
            base.as_os_str(),
            "ms-playwright".as_ref(),
            "playwright-rust".as_ref(),
            "driver".as_ref(),
        ]
        .iter()
        .collect();
        dir
    }

    pub fn platform(&self) -> Platform {
        match Self::PLATFORM {
            "linux" => Platform::Linux,
            "mac" => Platform::Mac,
            "win32" => Platform::Win32,
            "win32_x64" => Platform::Win32x64,
            _ => unreachable!(),
        }
    }

    pub fn executable(&self) -> PathBuf {
        // For Playwright 1.50+, we use node + package/cli.js
        // The old playwright.sh/playwright.cmd are no longer included
        match self.platform() {
            Platform::Linux | Platform::Mac => self.path.join("node"),
            Platform::Win32 | Platform::Win32x64 => self.path.join("node.exe"),
        }
    }

    /// Returns the path to the CLI script for Playwright 1.50+
    pub fn cli_script(&self) -> PathBuf {
        self.path.join("package").join("cli.js")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Platform {
    Linux,
    Win32,
    Win32x64,
    Mac,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn install() {
        let _driver = Driver::install().unwrap();
    }
}

pub use crate::imp::playwright::DeviceDescriptor;
use crate::{
    api::{browser_type::BrowserType, selectors::Selectors},
    imp::{core::*, playwright::Playwright as Impl, prelude::*},
    Error,
};
use std::{io, process::Command};

/// Entry point
pub struct Playwright {
    driver: Driver,
    _conn: Connection,
    inner: Weak<Impl>,
}

fn run(driver: &Driver, args: &'static [&'static str]) -> io::Result<()> {
    // For Playwright 1.50+, we run: node package/cli.js <args>
    let cli_script = driver.cli_script();
    let status = Command::new(driver.executable())
        .arg(&cli_script)
        .args(args)
        .status()?;
    if !status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Exit with {}", status),
        ));
    }
    Ok(())
}

impl Playwright {
    /// Installs the Playwright driver and initializes a new Playwright instance.
    ///
    /// This is the main entry point for using Playwright. It downloads and installs
    /// the Playwright driver to the system's cache directory
    /// (`$CACHE_DIR/.ms-playwright/playwright-rust/driver`) and establishes a connection
    /// to the driver process.
    ///
    /// # Returns
    ///
    /// Returns a new `Playwright` instance if the driver installation and connection succeed,
    /// or an `Error` if installation or connection fails.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The driver installation fails (network issues, disk space, etc.)
    /// - The connection to the driver process cannot be established
    ///
    /// # Examples
    ///
    /// Initialize Playwright and use it to launch a browser:
    ///
    /// ```
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// use playwright::Playwright;
    ///
    /// let playwright = Playwright::initialize().await?;
    /// let chromium = playwright.chromium();
    /// # Ok(())
    /// # }
    /// ```
    pub async fn initialize() -> Result<Playwright, Error> {
        let driver = Driver::install()?;
        Self::with_driver(driver).await
    }

    /// Constructs a new `Playwright` instance from an already-installed driver.
    ///
    /// Use this method when you have a custom `Driver` instance or want to use
    /// an existing driver installation. This establishes a connection to the driver
    /// process and initializes the Playwright object.
    ///
    /// # Arguments
    ///
    /// * `driver` - An installed Playwright driver instance
    ///
    /// # Returns
    ///
    /// Returns a new `Playwright` instance connected to the given driver, or an `Error`
    /// if the connection cannot be established.
    ///
    /// # Errors
    ///
    /// This function will return an error if the connection to the driver process fails.
    ///
    /// # Examples
    ///
    /// ```
    /// # async fn example() -> Result<(), playwright::Error> {
    /// use playwright::{Playwright, Driver};
    ///
    /// let driver = Driver::install()?;
    /// let playwright = Playwright::with_driver(driver).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn with_driver(driver: Driver) -> Result<Playwright, Error> {
        let conn = Connection::run(&driver)?;
        let p = Impl::wait_initial_object(&conn).await?;
        Ok(Self {
            driver,
            _conn: conn,
            inner: p,
        })
    }

    /// Installs all Playwright browsers (Chromium, Firefox, and WebKit).
    ///
    /// This runs the equivalent of `playwright install` and downloads all supported
    /// browser engines. It may take several minutes depending on your internet connection.
    /// If you only need specific browsers, consider using [`install_chromium`](Self::install_chromium),
    /// [`install_firefox`](Self::install_firefox), or [`install_webkit`](Self::install_webkit) instead.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if installation succeeds, or an `io::Error` if it fails.
    ///
    /// # Examples
    ///
    /// ```
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # use playwright::Playwright;
    /// # let playwright = Playwright::initialize().await?;
    /// playwright.prepare()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn prepare(&self) -> io::Result<()> {
        run(&self.driver, &["install"])
    }

    /// Installs the Chromium browser engine.
    ///
    /// This runs the equivalent of `playwright install chromium` and downloads
    /// the Chromium browser engine. If you only need Chromium, use this instead
    /// of [`prepare`](Self::prepare) to save disk space and download time.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if installation succeeds, or an `io::Error` if it fails.
    ///
    /// # Examples
    ///
    /// ```
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # use playwright::Playwright;
    /// # let playwright = Playwright::initialize().await?;
    /// playwright.install_chromium()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn install_chromium(&self) -> io::Result<()> {
        run(&self.driver, &["install", "chromium"])
    }

    /// Installs the Firefox browser engine.
    ///
    /// This runs the equivalent of `playwright install firefox` and downloads
    /// the Firefox browser engine. If you only need Firefox, use this instead
    /// of [`prepare`](Self::prepare) to save disk space and download time.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if installation succeeds, or an `io::Error` if it fails.
    ///
    /// # Examples
    ///
    /// ```
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # use playwright::Playwright;
    /// # let playwright = Playwright::initialize().await?;
    /// playwright.install_firefox()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn install_firefox(&self) -> io::Result<()> {
        run(&self.driver, &["install", "firefox"])
    }

    /// Installs the WebKit browser engine.
    ///
    /// This runs the equivalent of `playwright install webkit` and downloads
    /// the WebKit browser engine. If you only need WebKit, use this instead
    /// of [`prepare`](Self::prepare) to save disk space and download time.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if installation succeeds, or an `io::Error` if it fails.
    ///
    /// # Examples
    ///
    /// ```
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # use playwright::Playwright;
    /// # let playwright = Playwright::initialize().await?;
    /// playwright.install_webkit()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn install_webkit(&self) -> io::Result<()> {
        run(&self.driver, &["install", "webkit"])
    }

    /// Returns a launcher for the Chromium browser engine.
    ///
    /// # Returns
    ///
    /// A `BrowserType` instance that can be used to launch Chromium browsers.
    ///
    /// # Examples
    ///
    /// ```
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # use playwright::Playwright;
    /// # let playwright = Playwright::initialize().await?;
    /// let chromium = playwright.chromium();
    /// # Ok(())
    /// # }
    /// ```
    pub fn chromium(&self) -> BrowserType {
        match self.inner.upgrade() {
            Some(playwright_impl) => {
                let chromium_weak = playwright_impl.chromium();
                BrowserType::new(chromium_weak)
            }
            None => BrowserType::new(Weak::new()),
        }
    }

    /// Returns a launcher for the Firefox browser engine.
    ///
    /// # Returns
    ///
    /// A `BrowserType` instance that can be used to launch Firefox browsers.
    ///
    /// # Examples
    ///
    /// ```
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # use playwright::Playwright;
    /// # let playwright = Playwright::initialize().await?;
    /// let firefox = playwright.firefox();
    /// # Ok(())
    /// # }
    /// ```
    pub fn firefox(&self) -> BrowserType {
        let inner = weak_and_then(&self.inner, |rc| rc.firefox());
        BrowserType::new(inner)
    }

    /// Returns a launcher for the WebKit browser engine.
    ///
    /// # Returns
    ///
    /// A `BrowserType` instance that can be used to launch WebKit browsers.
    ///
    /// # Examples
    ///
    /// ```
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # use playwright::Playwright;
    /// # let playwright = Playwright::initialize().await?;
    /// let webkit = playwright.webkit();
    /// # Ok(())
    /// # }
    /// ```
    pub fn webkit(&self) -> BrowserType {
        let inner = weak_and_then(&self.inner, |rc| rc.webkit());
        BrowserType::new(inner)
    }

    /// Returns a mutable reference to the underlying `Driver`.
    ///
    /// This allows you to access driver-specific operations or configuration.
    ///
    /// # Returns
    ///
    /// A mutable reference to the `Driver` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # use playwright::Playwright;
    /// # let mut playwright = Playwright::initialize().await?;
    /// let driver = playwright.driver();
    /// # Ok(())
    /// # }
    /// ```
    pub fn driver(&mut self) -> &mut Driver {
        &mut self.driver
    }

    /// Returns a Selectors object for registering custom selector engines, if available.
    ///
    /// Note: Selectors are not available in Playwright 1.50+ as a separate object.
    /// This method returns `None` if selectors are not available in the current
    /// driver version.
    ///
    /// # Returns
    ///
    /// Some(`Selectors`) if available, or `None` if not supported by the driver.
    ///
    /// # Examples
    ///
    /// ```
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # use playwright::Playwright;
    /// # let playwright = Playwright::initialize().await?;
    /// if let Some(_selectors) = playwright.selectors() {
    ///     // Register custom selector engines
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn selectors(&self) -> Option<Selectors> {
        let inner = self.inner.upgrade()?;
        let selectors_weak = inner.selectors()?;
        Some(Selectors::new(selectors_weak))
    }

    /// Returns a dictionary of all available device descriptors.
    ///
    /// Device descriptors contain pre-configured settings for various devices
    /// (phones, tablets, etc.) that can be used with [`Browser::newContext`]
    /// or [`Browser::newPage`] to simulate real device behavior.
    ///
    /// # Returns
    ///
    /// A vector of `DeviceDescriptor` objects, or an empty vector if no devices
    /// are available.
    ///
    /// # Examples
    ///
    /// ```
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # use playwright::Playwright;
    /// # let playwright = Playwright::initialize().await?;
    /// let devices = playwright.devices();
    /// println!("Available devices: {}", devices.len());
    /// # Ok(())
    /// # }
    /// ```
    pub fn devices(&self) -> Vec<DeviceDescriptor> {
        upgrade(&self.inner)
            .map(|x| x.devices().to_vec())
            .unwrap_or_default()
    }

    /// Returns a device descriptor by name, if available.
    ///
    /// Device descriptors contain pre-configured settings for various devices
    /// (phones, tablets, etc.) that can be used with [`Browser::newContext`]
    /// or [`Browser::newPage`] to simulate real device behavior.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the device (e.g., "iPhone 12", "Pixel 5")
    ///
    /// # Returns
    ///
    /// Some(`DeviceDescriptor`) if a device with the given name exists,
    /// or `None` if no such device is found.
    ///
    /// # Examples
    ///
    /// ```
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # use playwright::Playwright;
    /// # let playwright = Playwright::initialize().await?;
    /// if let Some(_device) = playwright.device("iPhone 12") {
    ///     println!("Found iPhone 12");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn device(&self, name: &str) -> Option<DeviceDescriptor> {
        let inner = self.inner.upgrade()?;
        let device = inner.device(name)?;
        Some(device.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    crate::runtime_test!(failure_status_code, {
        let mut p = Playwright::initialize().await.unwrap();
        let err = run(p.driver(), &["nonExistentArg"]);
        assert!(err.is_err());
        if let Some(e) = err.err() {
            assert_eq!(e.kind(), io::ErrorKind::Other);
        }
    });
}

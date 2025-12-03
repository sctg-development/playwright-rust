//! # Playwright for Rust
//!
//! Playwright is a modern browser automation library that provides a high-level API to control
//! Chromium, Firefox, and WebKit browsers. This crate is a Rust port of the Playwright
//! JavaScript library.
//!
//! ## Features
//!
//! - **Cross-browser support**: Automate Chromium, Firefox, and WebKit browsers
//! - **Headless and headed modes**: Run browsers with or without a UI
//! - **Device emulation**: Simulate real devices like iPhone, iPad, and Android phones
//! - **Network control**: Monitor, intercept, and mock network requests
//! - **Screenshot and video recording**: Capture screenshots and record videos of your automation
//! - **Multiple runtimes**: Support for `tokio`, `actix-rt`, and `async-std`
//!
//! ## Getting Started
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! playwright = { url = "https://github.com/sctg-development/playwright-rust", branch = "master" }
//! tokio = { version = "1", features = ["full"] }
//! ```
//!
//! ## Basic Example
//!
//! This example initializes Playwright, launches a browser, navigates to a website,
//! and retrieves the version number:
//!
//! ```
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! use playwright::Playwright;
//!
//! // Initialize Playwright and install browsers if needed
//! let playwright = Playwright::initialize().await?;
//! playwright.prepare()?;
//!
//! // Launch a headless Chromium browser
//! let chromium = playwright.chromium();
//! let browser = chromium.launcher().headless(true).launch().await?; // Use .headless(false) to see the browser
//!
//! // Create a new browser context and page
//! let context = browser.context_builder().build().await?;
//! let page = context.new_page().await?;
//!
//! // Navigate to the GitHub Pages documentation
//! page.goto_builder("https://sctg-development.github.io/playwright-rust/playwright/")
//!     .goto()
//!     .await?;
//!
//! // Execute JavaScript to get the current URL
//! let url: String = page.eval("() => location.href").await?;
//! println!("Current URL: {}", url);
//!
//! // Click on the API documentation link
//! page.click_builder(r#"a[title="mod playwright::api"]"#)
//!     .click()
//!     .await?;
//!
//! // Extract the version number from the documentation page
//! let version: String = page
//!     .eval(r#"() => document.querySelector("span.version").innerText.trim()"#)
//!     .await?;
//! println!("Package version: {}", version);
//!
//! // Verify we're on the correct page
//! assert_eq!(
//!     page.url().unwrap(),
//!     "https://sctg-development.github.io/playwright-rust/playwright/api/index.html"
//! );
//!
//! // Clean up - browser context and page are automatically closed when dropped
//! browser.close().await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Async Runtimes
//!
//! This crate supports multiple async runtimes via feature flags:
//!
//! - `rt-tokio` (default): Uses the `tokio` runtime
//! - `rt-actix`: Uses the `actix-rt` runtime
//! - `rt-async-std`: Uses the `async-std` runtime
//!
//! ## Main Types
//!
//! - [`Playwright`]: The main entry point for browser automation
//! - [`api::browser_type::BrowserType`]: Launcher for a specific browser engine
//! - [`api::browser::Browser`]: A browser instance
//! - [`api::browser_context::BrowserContext`]: A context in which pages are isolated
//! - [`api::page::Page`]: A single tab or window in a browser context
//!
//! ## Resources
//!
//! - [GitHub Repository](https://github.com/sctg-development/playwright-rust)
//! - [Official Playwright Documentation](https://playwright.dev)

#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_with;

pub mod api;
mod imp;

pub use crate::imp::core::{Driver, Error};
pub use api::playwright::Playwright;

#[doc(hidden)]
#[macro_export]
macro_rules! runtime_test {
    ($name:tt, $main:stmt) => {
        #[cfg(feature = "rt-tokio")]
        #[test]
        fn $name() {
            env_logger::builder().is_test(true).try_init().ok();
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async { $main });
        }

        #[cfg(feature = "rt-actix")]
        #[test]
        fn $name() {
            env_logger::builder().is_test(true).try_init().ok();
            actix_rt::System::new().block_on(async { $main });
        }

        #[cfg(feature = "rt-async-std")]
        #[test]
        fn $name() {
            env_logger::builder().is_test(true).try_init().ok();
            async_std::task::block_on(async { $main });
        }
    };
}

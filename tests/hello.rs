use playwright::Playwright;

playwright::runtime_test!(hello, {
    main().await.unwrap();
});

async fn main() -> Result<(), playwright::Error> {
    let playwright = Playwright::initialize().await?; // if drop all resources are disposed
    playwright.prepare()?; // install browsers
    let chromium = playwright.chromium();
    let browser = chromium.launcher().headless(true).launch().await?;
    let context = browser.context_builder().build().await?;
    let page = context.new_page().await?;

    page.goto_builder("https://sctg-development.github.io/playwright-rust/playwright/")
        .goto()
        .await?;
    // Exec js on browser and Deserialize with serde
    let url: String = page.eval("() => location.href").await?;
    assert_eq!(
        url,
        "https://sctg-development.github.io/playwright-rust/playwright/"
    );

    // Wait until navigated
    page.click_builder(r#"a[title="mod playwright::api"]"#)
        .click()
        .await?;

    // Check if span with class=version contains package version
    let version: String = page
        .eval(&format!(
            r#"() => document.querySelector("span.version").innerText.trim()"#
        ))
        .await?;
    assert_eq!(version, format!("{}", env!("CARGO_PKG_VERSION")));
    assert_eq!(
        page.url().unwrap(),
        "https://sctg-development.github.io/playwright-rust/playwright/api/index.html"
    );

    Ok(())
}

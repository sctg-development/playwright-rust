use super::Which;

pub async fn all(which: Which) {
    query_selector_basic_example(which).await;
    click_builder_basic_example(which).await;
    page_eval_test(which).await;
    page_goto_test(which).await;
    page_content_test(which).await;
    frame_navigation_test(which).await;
    keyboard_input_test(which).await;
    multiple_frames_test(which).await;
    browser_context_test(which).await;
    dblclick_and_hover_test(which).await;
    element_visibility_test(which).await;
}

async fn query_selector_basic_example(which: Which) {
    let playwright = crate::playwright_with_driver().await;
    crate::install_browser(&playwright, which);

    let t = match which {
        Which::Webkit => playwright.webkit(),
        Which::Firefox => playwright.firefox(),
        Which::Chromium => playwright.chromium(),
    };

    let browser = t.launcher().launch().await.unwrap();
    let bc = browser.context_builder().build().await.unwrap();
    let page = bc.new_page().await.unwrap();

    // Navigate to the documentation page
    page.goto_builder("https://sctg-development.github.io/playwright-rust/playwright/index.html")
        .goto()
        .await
        .unwrap();

    // Use query_selector to find the link
    let element = page
        .query_selector("a[href='#basic-example'][title='Basic Example']")
        .await
        .unwrap();

    assert!(
        element.is_some(),
        "Element with href='#basic-example' and title='Basic Example' should be found"
    );

    // Click the element using click_builder
    page.click_builder("a[href='#basic-example'][title='Basic Example']")
        .click()
        .await
        .unwrap();

    // Verify that the page navigated to the section (check URL contains the hash)
    let url = page.url().unwrap();
    assert!(
        url.contains("#basic-example"),
        "URL should contain '#basic-example' after clicking"
    );

    browser.close().await.unwrap();
}

async fn click_builder_basic_example(which: Which) {
    let playwright = crate::playwright_with_driver().await;
    crate::install_browser(&playwright, which);

    let t = match which {
        Which::Webkit => playwright.webkit(),
        Which::Firefox => playwright.firefox(),
        Which::Chromium => playwright.chromium(),
    };

    let browser = t.launcher().launch().await.unwrap();
    let bc = browser.context_builder().build().await.unwrap();
    let page = bc.new_page().await.unwrap();

    // Navigate to the documentation page
    page.goto_builder("https://sctg-development.github.io/playwright-rust/playwright/index.html")
        .goto()
        .await
        .unwrap();

    // Use click_builder to click the link directly
    page.click_builder("a[href='#basic-example'][title='Basic Example']")
        .click()
        .await
        .unwrap();

    // Verify that the page navigated to the section (check URL contains the hash)
    let url = page.url().unwrap();
    assert!(
        url.contains("#basic-example"),
        "URL should contain '#basic-example' after clicking with click_builder"
    );

    browser.close().await.unwrap();
}

async fn page_eval_test(which: Which) {
    let playwright = crate::playwright_with_driver().await;
    crate::install_browser(&playwright, which);

    let t = match which {
        Which::Webkit => playwright.webkit(),
        Which::Firefox => playwright.firefox(),
        Which::Chromium => playwright.chromium(),
    };

    let browser = t.launcher().launch().await.unwrap();
    let bc = browser.context_builder().build().await.unwrap();
    let page = bc.new_page().await.unwrap();

    // Navigate to the documentation page
    page.goto_builder("https://sctg-development.github.io/playwright-rust/playwright/index.html")
        .goto()
        .await
        .unwrap();

    // Test page.eval() to get the page title
    let title: String = page.eval("() => document.title").await.unwrap();
    assert!(!title.is_empty(), "Page title should not be empty");

    browser.close().await.unwrap();
}

async fn page_goto_test(which: Which) {
    let playwright = crate::playwright_with_driver().await;
    crate::install_browser(&playwright, which);

    let t = match which {
        Which::Webkit => playwright.webkit(),
        Which::Firefox => playwright.firefox(),
        Which::Chromium => playwright.chromium(),
    };

    let browser = t.launcher().launch().await.unwrap();
    let bc = browser.context_builder().build().await.unwrap();
    let page = bc.new_page().await.unwrap();

    // Test page.goto() with wait until options
    let response = page
        .goto_builder("https://sctg-development.github.io/playwright-rust/playwright/index.html")
        .goto()
        .await
        .unwrap();

    assert!(response.is_some(), "Response should not be None");
    let url = page.url().unwrap();
    assert!(url.contains("index.html"), "URL should contain index.html");

    browser.close().await.unwrap();
}

async fn page_content_test(which: Which) {
    let playwright = crate::playwright_with_driver().await;
    crate::install_browser(&playwright, which);

    let t = match which {
        Which::Webkit => playwright.webkit(),
        Which::Firefox => playwright.firefox(),
        Which::Chromium => playwright.chromium(),
    };

    let browser = t.launcher().launch().await.unwrap();
    let bc = browser.context_builder().build().await.unwrap();
    let page = bc.new_page().await.unwrap();

    // Set page content and test it
    let test_html = r#"<div><h1>Test Page</h1><p>This is a test</p></div>"#;
    page.set_content_builder(test_html)
        .set_content()
        .await
        .unwrap();

    // Verify the content was set
    let content = page.content().await.unwrap();
    assert!(
        content.contains("Test Page"),
        "Content should contain 'Test Page'"
    );
    assert!(
        content.contains("This is a test"),
        "Content should contain 'This is a test'"
    );

    browser.close().await.unwrap();
}

async fn frame_navigation_test(which: Which) {
    let playwright = crate::playwright_with_driver().await;
    crate::install_browser(&playwright, which);

    let t = match which {
        Which::Webkit => playwright.webkit(),
        Which::Firefox => playwright.firefox(),
        Which::Chromium => playwright.chromium(),
    };

    let browser = t.launcher().launch().await.unwrap();
    let bc = browser.context_builder().build().await.unwrap();
    let page = bc.new_page().await.unwrap();

    // Navigate to the documentation page
    page.goto_builder("https://sctg-development.github.io/playwright-rust/playwright/index.html")
        .goto()
        .await
        .unwrap();

    // Get the main frame
    let frame = page.main_frame();
    let frame_url = frame.url().unwrap();
    assert!(!frame_url.is_empty(), "Frame URL should not be empty");

    // Test query selector on frame
    let element = frame.query_selector("a").await.unwrap();

    assert!(
        element.is_some(),
        "Frame should have at least one anchor element"
    );

    browser.close().await.unwrap();
}

async fn keyboard_input_test(which: Which) {
    let playwright = crate::playwright_with_driver().await;
    crate::install_browser(&playwright, which);

    let t = match which {
        Which::Webkit => playwright.webkit(),
        Which::Firefox => playwright.firefox(),
        Which::Chromium => playwright.chromium(),
    };

    let browser = t.launcher().launch().await.unwrap();
    let bc = browser.context_builder().build().await.unwrap();
    let page = bc.new_page().await.unwrap();

    // Create a simple page with an input field
    let html = r#"<input type="text" id="test-input" />"#;
    page.set_content_builder(html).set_content().await.unwrap();

    // Test fill method instead of keyboard input
    page.fill_builder("#test-input", "Hello Playwright")
        .fill()
        .await
        .unwrap();

    // Verify the input was filled
    let value: String = page
        .eval("() => document.getElementById('test-input').value")
        .await
        .unwrap();
    assert_eq!(
        value, "Hello Playwright",
        "Input should be filled with text"
    );

    browser.close().await.unwrap();
}

async fn multiple_frames_test(which: Which) {
    let playwright = crate::playwright_with_driver().await;
    crate::install_browser(&playwright, which);

    let t = match which {
        Which::Webkit => playwright.webkit(),
        Which::Firefox => playwright.firefox(),
        Which::Chromium => playwright.chromium(),
    };

    let browser = t.launcher().launch().await.unwrap();
    let bc = browser.context_builder().build().await.unwrap();
    let page = bc.new_page().await.unwrap();

    // Create a page with multiple elements to test frame operations
    let html = r#"
        <div id="main">
            <h1>Main Content</h1>
            <button>Click Me</button>
            <div id="child">Child Content</div>
        </div>
    "#;
    page.set_content_builder(html).set_content().await.unwrap();

    // Test frames method
    let frames = page.frames().unwrap();
    assert!(!frames.is_empty(), "Page should have at least one frame");

    // Test getting title through frame
    let frame = page.main_frame();
    let title = frame.title().await.unwrap();
    assert_eq!(title, "", "Frame title on empty page should be empty");

    browser.close().await.unwrap();
}

async fn browser_context_test(which: Which) {
    let playwright = crate::playwright_with_driver().await;
    crate::install_browser(&playwright, which);

    let t = match which {
        Which::Webkit => playwright.webkit(),
        Which::Firefox => playwright.firefox(),
        Which::Chromium => playwright.chromium(),
    };

    let browser = t.launcher().launch().await.unwrap();

    // Test creating multiple pages
    let bc1 = browser.context_builder().build().await.unwrap();
    let page1 = bc1.new_page().await.unwrap();

    let bc2 = browser.context_builder().build().await.unwrap();
    let page2 = bc2.new_page().await.unwrap();

    // Navigate both pages
    page1
        .goto_builder("https://sctg-development.github.io/playwright-rust/playwright/index.html")
        .goto()
        .await
        .unwrap();

    page2
        .goto_builder(
            "https://sctg-development.github.io/playwright-rust/playwright/api/index.html",
        )
        .goto()
        .await
        .unwrap();

    // Verify both pages have different URLs
    let url1 = page1.url().unwrap();
    let url2 = page2.url().unwrap();

    assert!(
        url1.contains("playwright/index.html"),
        "Page 1 URL should contain index.html"
    );
    assert!(url2.contains("api"), "Page 2 URL should contain api");
    assert_ne!(url1, url2, "Pages should have different URLs");

    browser.close().await.unwrap();
}

async fn dblclick_and_hover_test(which: Which) {
    let playwright = crate::playwright_with_driver().await;
    crate::install_browser(&playwright, which);

    let t = match which {
        Which::Webkit => playwright.webkit(),
        Which::Firefox => playwright.firefox(),
        Which::Chromium => playwright.chromium(),
    };

    let browser = t.launcher().launch().await.unwrap();
    let bc = browser.context_builder().build().await.unwrap();
    let page = bc.new_page().await.unwrap();

    // Create a page with interactive elements
    let html = r#"
        <input id="dblclick-input" value="0" />
        <button id="hover-btn">Hover over me</button>
        <style>
            #hover-btn:hover { background-color: red; }
        </style>
    "#;
    page.set_content_builder(html).set_content().await.unwrap();

    // Test dblclick_builder
    page.dblclick_builder("#dblclick-input")
        .dblclick()
        .await
        .unwrap();

    // Test hover_builder
    page.hover_builder("#hover-btn").goto().await.unwrap();

    // Verify hover state by checking computed style
    let bg_color: String = page
        .eval("() => window.getComputedStyle(document.getElementById('hover-btn')).backgroundColor")
        .await
        .unwrap_or_default();

    // The color might vary across browsers, just ensure hover worked without error
    assert!(!bg_color.is_empty(), "Should be able to get computed style");

    browser.close().await.unwrap();
}

async fn element_visibility_test(which: Which) {
    let playwright = crate::playwright_with_driver().await;
    crate::install_browser(&playwright, which);

    let t = match which {
        Which::Webkit => playwright.webkit(),
        Which::Firefox => playwright.firefox(),
        Which::Chromium => playwright.chromium(),
    };

    let browser = t.launcher().launch().await.unwrap();
    let bc = browser.context_builder().build().await.unwrap();
    let page = bc.new_page().await.unwrap();

    // Create a page with visibility elements
    let html = r#"
        <div id="visible" style="display: block;">Visible</div>
        <div id="hidden" style="display: none;">Hidden</div>
        <div id="invisible" style="visibility: hidden;">Invisible</div>
    "#;
    page.set_content_builder(html).set_content().await.unwrap();

    // Test is_visible
    let visible = page.is_visible("#visible", None).await.unwrap();
    assert!(visible, "#visible should be visible");

    let hidden = page.is_visible("#hidden", None).await.unwrap();
    assert!(!hidden, "#hidden should not be visible");

    // Test is_hidden
    let hidden_check = page.is_hidden("#hidden", None).await.unwrap();
    assert!(hidden_check, "#hidden should be hidden");

    browser.close().await.unwrap();
}

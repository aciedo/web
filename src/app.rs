use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/web.css"/>
        <Stylesheet href="/inter.css"/>

        // sets the document title
        <Title text="Valera"/>

        // content for this welcome page
        <Router fallback=|cx| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { cx,
                <ErrorTemplate outside_errors/>
            }
            .into_view(cx)
        }>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="bar"/>
        <div class="content">
            <header>
            <A href="/">
                <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 32 32" fill="none">
                    <path d="M12.671 32H19.4224V21.8266L28.2086 26.9133L31.538 21.0867L22.7519 16L31.538 10.9133L28.2086 5.08671L19.4224 10.1734V0H12.671V10.1734L3.79237 5.08671L0.462891 10.9133L9.24902 16L0.462891 21.0867L3.79237 26.9133L12.671 21.8266V32Z" fill="white"/>
                </svg>
                </A>
            </header>
            <div class="main-content">
                <h1 class="title">"Finance. Reimagined."<br/>"Coming soon."</h1>
            </div>
            <footer>
            <div class="left-footer">
            <p class="footer-text">"22nd century finance for all"</p>
            </div>
            <div class="right-footer">
            <p class="footer-text">"© 2123 Valera"</p>
            </div>
            </footer>
        </div>
        <div class="bar"/>
    }
}

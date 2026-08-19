use leptos::*;
use leptos_router::A;

#[component]
pub fn RerunPage() -> impl IntoView {
    let rerun_url = "https://app.rerun.io/version/0.36.0/index.html?url=https%3A%2F%2Fapp.rerun.io%2Fversion%2F0.36.0%2Fexamples%2Fnuscenes_dataset.rrd";

    view! {
        <main class="rerun-page">
            <header class="rerun-header">
                <A href="/" class="back-link">
                    "← Back"
                </A>
                <h1>"Rerun camera visualization"</h1>
            </header>
            <div class="viewer-container">
                <iframe
                    src=rerun_url
                    title="Rerun camera visualization"
                    allow="accelerometer; camera; gyroscope; vr; xr-spatial-tracking; fullscreen"
                ></iframe>
            </div>
        </main>
    }
}

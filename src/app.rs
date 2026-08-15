use crate::components::graphics::GraphicsCanvas;
use crate::pages::home::HomePage;
use crate::pages::rerun::RerunPage;
use leptos::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <div class="app">
                <GraphicsCanvas />
                <Routes>
                    <Route path="" view=HomePage />
                    <Route path="/rerun-camera-visualization" view=RerunPage />
                    <Route path="/rerun" view=RerunPage />
                    <Route path="/*any" view=HomePage />
                </Routes>
            </div>
        </Router>
    }
}

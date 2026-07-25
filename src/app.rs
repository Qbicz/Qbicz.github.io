use crate::components::graphics::GraphicsCanvas;
use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="app">
            <GraphicsCanvas />
            <main class="content">
                <section class="hero">
                    <h1>"Hi, I'm Filip, an engineer that cares"</h1>
                    <p class="subtitle">
                        "I build products that matter, thoughtfully and reliably. I believe the best solutions come from listening, genuine interest in the technology, and care for the people you help."
                    </p>
                    <p class="subtitle">
                        "Join me in the journey of learning and curiosity."
                    </p>
                    <div class="links">
                        <a href="https://github.com/Qbicz" target="_blank" rel="noopener">
                            "GitHub"
                        </a>
                        <a href="https://linkedin.com/in/kubicz" target="_blank" rel="noopener">
                            "LinkedIn"
                        </a>
                        <a href="https://linktr.ee/filipkubicz" target="_blank" rel="noopener">
                            "Linktree"
                        </a>
                    </div>
                </section>
            </main>
        </div>
    }
}

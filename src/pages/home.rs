use leptos::*;
use leptos_router::A;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
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
                </div>
            </section>

            <footer class="bottom-section">
                <a href="https://kubicz.engineer/fast-reading-app/" class="subpage-link">
                    "Fast reading app"
                </a>
                <A href="/rerun-camera-visualization" class="subpage-link">
                    "Rerun camera visualization"
                </A>
            </footer>
        </main>
    }
}

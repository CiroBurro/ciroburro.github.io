pub mod portfolio;
pub mod scoundrel;
pub mod theme;

use leptos::prelude::*;
use leptos_router::{components::*, path};
use portfolio::PortfolioPage;
use scoundrel::ui::ScoundrelPage;
use theme::{DividerOrnate, NoiseOverlay};

fn main() {
    leptos::mount::mount_to_body(|| view! { <App /> });
}

#[component]
fn App() -> impl IntoView {
    view! {
        <NoiseOverlay />
        <Router>
            <div class="relative z-10 flex min-h-screen flex-col">
                <Nav />
                <main class="flex-1">
                    <Routes fallback=|| view! { <NotFound /> }>
                        <Route path=path!("") view=PortfolioPage />
                        <Route path=path!("/scoundrel") view=ScoundrelPage />
                    </Routes>
                </main>
                <Footer />
            </div>
        </Router>
    }
}

#[component]
fn Nav() -> impl IntoView {
    view! {
        <header class="sticky top-0 z-40 border-b border-gold-500/25 bg-ink-950/85 backdrop-blur-md">
            <nav class="mx-auto flex max-w-5xl items-center justify-between px-5 py-3">
                <A
                    href=""
                    attr:class="font-display text-lg font-bold tracking-[0.22em] text-gold-300 uppercase hover:text-gold-200 transition-colors"
                >
                    "Filippo Baglioni"
                </A>
                <div class="flex items-center gap-6 text-sm">
                    <A
                        href=""
                        attr:class="font-display uppercase tracking-[0.18em] text-parchment-200 hover:text-gold-300 transition-colors"
                    >
                        "Portfolio"
                    </A>
                    <A
                        href="/scoundrel"
                        attr:class="font-display uppercase tracking-[0.18em] text-parchment-200 hover:text-gold-300 transition-colors"
                    >
                        "Scoundrel"
                    </A>
                </div>
            </nav>
        </header>
    }
}

#[component]
fn Footer() -> impl IntoView {
    view! {
        <footer class="mt-16 border-t border-gold-500/20 bg-ink-950/70">
            <div class="mx-auto max-w-5xl px-5 py-8 text-center">
                <DividerOrnate />
                <p class="font-display text-xs tracking-[0.3em] uppercase text-parchment-500">
                    "Forged with Rust • Leptos • Tailwind"
                </p>
            </div>
        </footer>
    }
}

#[component]
fn NotFound() -> impl IntoView {
    view! {
        <div class="mx-auto max-w-2xl px-5 py-24 text-center">
            <p class="font-display text-7xl text-wine-500">"404"</p>
            <p class="mt-4 font-display text-xl tracking-[0.2em] uppercase text-gold-300">
                "You got lost in the dungeon"
            </p>
            <p class="mt-2 text-parchment-300">
                "This room doesn't exist. Best to retrace your steps before the goblins find you."
            </p>
            <a href="https://ciroburro.github.io/" class="btn btn-gold mt-8 inline-block">"Back to the light"</a>
        </div>
    }
}

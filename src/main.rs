pub mod portfolio;
pub mod scoundrel;

use leptos::prelude::*;
use leptos_router::{components::*, path};
use portfolio::PortfolioPage;
use scoundrel::ui::ScoundrelPage;

fn main() {
    leptos::mount::mount_to_body(|| view! { <p>"Hello, Leptos!"</p> });
}


#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <nav class="bg-slate-800 p-4 flex gap-4 text-white">
                <A href="" attr:class="hover:underline">"Home / Portfolio"</A>
                <A href="/scoundrel" attr:class="hover:underline">"Gioca a Scoundrel"</A>
            </nav>

            <main>
                <Routes fallback=|| view! { <p class="text-white">"Pagina non trovata (404)"</p> }>
                    <Route path=path!("") view=PortfolioPage />

                    <Route path=path!("/scoundrel") view=ScoundrelPage />
                </Routes>
            </main>
        </Router>
    }
}
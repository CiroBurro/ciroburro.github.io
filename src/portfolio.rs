use crate::theme::{Badge, CardBack, DividerOrnate, OrnateFrame};
use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn PortfolioPage() -> impl IntoView {
    view! {
        <div class="mx-auto max-w-5xl px-5">
            <Hero />
            <DividerOrnate />
            <Projects />
            <DividerOrnate />
            <Skills />
            <DividerOrnate />
            <Contact />
        </div>
    }
}

#[component]
fn Hero() -> impl IntoView {
    view! {
        <section class="grid items-center gap-12 py-16 md:grid-cols-[1.4fr_1fr] md:py-24">
            <div>
                <h1 class="font-display text-5xl font-bold leading-tight text-gold-200 uppercase md:text-7xl">
                    "CiroBurro"
                </h1>
                <p class="mt-4 font-display text-lg tracking-[0.14em] text-parchment-300 uppercase">
                    "IT enthusiast"
                </p>
                <p class="mt-6 max-w-xl text-lg leading-relaxed text-parchment-200">
                    "Welcome to my digital tavern. I'm a developer passionate about systems,
                    networks and low-level adventures. Here you'll find my projects and a
                    tribute to Scoundrel, the dungeon-crawling solitaire I brought to the
                    browser."
                </p>
                <div class="mt-10 flex flex-wrap gap-4">
                    <A href="/scoundrel" attr:class="btn btn-gold">"Enter the Dungeon"</A>
                    <a href="#projects" class="btn btn-outline">"Explore the projects"</a>
                </div>
            </div>

            <div class="relative hidden justify-center md:flex">
                <div class="relative h-72 w-52 rotate-6 transition-transform duration-500 hover:rotate-0">
                    <CardBack class="h-full w-full" />
                </div>
                <div class="absolute -left-2 top-16 h-72 w-52 -rotate-6 transition-transform duration-500 hover:rotate-0">
                    <CardBack class="h-full w-full" />
                </div>
                <div class="absolute inset-0 -z-10 rounded-full bg-wine-800/30 blur-3xl"></div>
            </div>
        </section>
    }
}

#[derive(Clone)]
struct ProjectData {
    title: &'static str,
    tag: &'static str,
    description: &'static str,
    chips: &'static [&'static str],
    cta_label: &'static str,
    cta_href: &'static str,
    external: bool,
}

const PROJECTS: &[ProjectData] = &[
    ProjectData {
        title: "Scoundrel Web",
        tag: "Featured",
        description: "The dungeon-crawling solitaire in web form: deck, armory and room in a single adventure. Reimplemented in Rust with Leptos.",
        chips: &["Rust", "Leptos", "Tailwind", "WebAssembly"],
        cta_label: "Play now",
        cta_href: "/scoundrel",
        external: false,
    },
    ProjectData {
        title: "Pirate",
        tag: "Rust",
        description: "A torrent client for treasure hunters — written in Rust, obviously.",
        chips: &["Torrent", "Networking"],
        cta_label: "View on GitHub",
        cta_href: "https://github.com/CiroBurro/Pirate",
        external: true,
    },
    ProjectData {
        title: "chip8-emu",
        tag: "C",
        description: "A dual-platform CHIP-8 emulator in C, running on desktop PCs and the M5Stack Cardputer (ESP32).",
        chips: &["Emulator", "ESP32"],
        cta_label: "View on GitHub",
        cta_href: "https://github.com/CiroBurro/chip8-emu",
        external: true,
    },
    ProjectData {
        title: "NeoMatrix",
        tag: "Rust",
        description: "A machine learning framework written in Rust, exposed to Python.",
        chips: &["Machine Learning", "Python"],
        cta_label: "View on GitHub",
        cta_href: "https://github.com/CiroBurro/NeoMatrix",
        external: true,
    },
    ProjectData {
        title: "Gravity",
        tag: "Rust",
        description: "A 3D solar system with realistic gravity simulation.",
        chips: &["3D", "Simulation"],
        cta_label: "View on GitHub",
        cta_href: "https://github.com/CiroBurro/Gravity",
        external: true,
    },
    ProjectData {
        title: "rustyransom",
        tag: "Go",
        description: "Proof of concept: a sophisticated ransomware written in Rust, embedded inside a dropper (trojan).",
        chips: &["Rust", "Security"],
        cta_label: "View on GitHub",
        cta_href: "https://github.com/CiroBurro/rustyransom",
        external: true,
    },
];

#[component]
fn Projects() -> impl IntoView {
    view! {
        <section id="projects" class="py-10">
            <h2 class="section-title text-3xl">"Projects"</h2>

            <div class="mt-10 grid gap-6 md:grid-cols-2">
                {PROJECTS
                    .iter()
                    .map(|p| {
                        view! {
                            <ProjectCard
                                title=p.title
                                tag=p.tag
                                description=p.description
                                chips=p.chips
                                cta_label=p.cta_label
                                cta_href=p.cta_href
                                external=p.external
                            />
                        }
                    })
                    .collect_view()}
            </div>
        </section>
    }
}

#[component]
fn ProjectCard(
    title: &'static str,
    tag: &'static str,
    description: &'static str,
    chips: &'static [&'static str],
    #[prop(optional)] cta_label: Option<&'static str>,
    #[prop(optional)] cta_href: Option<&'static str>,
    #[prop(optional)] external: bool,
) -> impl IntoView {
    let cta_view = match (cta_label, cta_href) {
        (Some(label), Some(href)) => view! {
            <a
                href=href
                class="btn btn-outline mt-1 inline-block"
                target=if external { "_blank" } else { "_self" }
                rel=if external { "noopener noreferrer" } else { "" }
            >
                {label}
            </a>
        }
        .into_any(),
        _ => view! {
            <span class="badge mt-1 inline-block opacity-70">"Coming soon"</span>
        }
        .into_any(),
    };

    view! {
        <article class="project-card flex flex-col p-6">
            <div class="flex items-center justify-between gap-4">
                <h3 class="font-display text-xl tracking-[0.12em] text-gold-200 uppercase">{title}</h3>
                <span class="badge">{tag}</span>
            </div>
            <p class="mt-4 flex-1 leading-relaxed text-parchment-200">{description}</p>
            <div class="mt-6 flex flex-wrap gap-2">
                {chips
                    .iter()
                    .copied()
                    .map(|c| view! { <Badge>{c}</Badge> })
                    .collect_view()}
            </div>
            <div class="mt-6">{cta_view}</div>
        </article>
    }
}

#[component]
fn Skills() -> impl IntoView {
    let languages = vec!["Rust", "C", "Java", "Python", "Bash", "Go"];
    let ops = vec!["Linux", "Docker", "Networking", "Nix"];

    view! {
        <section class="py-10">
            <h2 class="section-title text-3xl">"Skills"</h2>
            <div class="mt-8 grid gap-6 md:grid-cols-3">
                <SkillBox title="Languages" items=languages />
                <SkillBox title="Operations" items=ops />
            </div>
        </section>
    }
}

#[component]
fn SkillBox(title: &'static str, items: Vec<&'static str>) -> impl IntoView {
    view! {
        <OrnateFrame>
            <h3 class="font-display text-lg tracking-[0.14em] text-gold-300 uppercase">{title}</h3>
            <div class="mt-5 flex flex-wrap gap-2">
                {items
                    .into_iter()
                    .map(|s| view! { <Badge>{s}</Badge> })
                    .collect_view()}
            </div>
        </OrnateFrame>
    }
}

#[component]
fn Contact() -> impl IntoView {
    view! {
        <section class="py-10">
            <h2 class="section-title text-3xl">"Contact"</h2>
            <div class="mt-8">
                <OrnateFrame>
                    <p class="text-lg leading-relaxed text-parchment-200">
                        "If you have an interesting quest — or just want to share a virtual mead —
                        find me on GitHub or send me a letter."
                    </p>
                    <div class="mt-6 flex flex-wrap gap-4">
                        <a href="https://github.com/CiroBurro" class="btn btn-wine">"GitHub"</a>
                        <a href="https://www.linkedin.com/in/filippo-baglioni-852858344/" class="btn btn-wine">"LinkedIn"</a>
                        <a href="mailto:filobaglioni.06@proton.me" class="btn btn-outline">"Email me"</a>
                    </div>
                </OrnateFrame>
            </div>
        </section>
    }
}

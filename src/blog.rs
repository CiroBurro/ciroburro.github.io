use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_params_map;
use pulldown_cmark::{html, Options, Parser};

use crate::theme::DividerOrnate;

// AUTO-GENERATED: PostMeta struct + POSTS const from build.rs
include!(concat!(env!("OUT_DIR"), "/posts_generated.rs"));

// ── Helpers ──────────────────────────────────────────────────────────────────

fn strip_frontmatter(src: &str) -> &str {
    // Remove YAML frontmatter delimited by leading/trailing "---"
    let s = src.trim_start();
    if !s.starts_with("---") {
        return src;
    }
    let rest = &s[3..];
    if let Some(end) = rest.find("\n---") {
        rest[end + 4..].trim_start()
    } else {
        src
    }
}

fn md_to_html(md: &str) -> String {
    let body = strip_frontmatter(md);
    let parser = Parser::new_ext(body, Options::ENABLE_STRIKETHROUGH | Options::ENABLE_TABLES);
    let mut out = String::new();
    html::push_html(&mut out, parser);
    out
}

// ── Blog list ─────────────────────────────────────────────────────────────────

#[component]
pub fn BlogPage() -> impl IntoView {
    view! {
        <div class="mx-auto max-w-5xl px-5 py-16">
            <h1 class="section-title text-3xl">"Tavern Logbook"</h1>
            <p class="mt-4 text-parchment-300 leading-relaxed">
                "Notes on systems, security, and the occasional dungeon crawl."
            </p>
            <DividerOrnate />
            <ul class="mt-8 flex flex-col gap-6">
                {POSTS
                    .iter()
                    .map(|p| view! { <PostListItem slug=p.slug title=p.title date=p.date description=p.description /> })
                    .collect_view()}
            </ul>
        </div>
    }
}

#[component]
fn PostListItem(
    slug: &'static str,
    title: &'static str,
    date: &'static str,
    description: &'static str,
) -> impl IntoView {
    let href = format!("/blog/{slug}");
    view! {
        <li>
            <div class="plain-frame p-6">
                <div class="flex flex-col gap-2">
                    <span class="font-display text-xs tracking-[0.2em] uppercase text-parchment-500">{date}</span>
                    <h2 class="font-display text-xl tracking-[0.12em] text-gold-200 uppercase">{title}</h2>
                    <p class="text-parchment-200 leading-relaxed">{description}</p>
                    <A href=href attr:class="btn btn-outline mt-2 self-start">"Read →"</A>
                </div>
            </div>
        </li>
    }
}

// ── Single post ───────────────────────────────────────────────────────────────

#[component]
pub fn PostPage() -> impl IntoView {
    let params = use_params_map();
    let slug = move || params.read().get("slug").unwrap_or_default();

    let post = move || POSTS.iter().find(|p| p.slug == slug());

    view! {
        {move || match post() {
            None => view! {
                <div class="mx-auto max-w-2xl px-5 py-24 text-center">
                    <p class="font-display text-5xl text-wine-500">"404"</p>
                    <p class="mt-4 font-display text-xl tracking-[0.2em] uppercase text-gold-300">"Post not found"</p>
                    <A href="/blog" attr:class="btn btn-gold mt-8 inline-block">"Back to logbook"</A>
                </div>
            }.into_any(),
            Some(p) => {
                let html = md_to_html(p.body);
                view! {
                    <div class="mx-auto max-w-3xl px-5 py-16">
                        <A href="/blog" attr:class="font-display text-sm uppercase tracking-[0.18em] text-parchment-400 hover:text-gold-300 transition-colors">"← Logbook"</A>
                        <h1 class="mt-6 font-display text-4xl font-bold leading-tight text-gold-200 uppercase">{p.title}</h1>
                        <p class="mt-2 font-display text-xs tracking-[0.2em] uppercase text-parchment-500">{p.date}</p>
                        <DividerOrnate />
                        <div class="prose-dungeon mt-8" inner_html=html />
                    </div>
                }.into_any()
            }
        }}
    }
}

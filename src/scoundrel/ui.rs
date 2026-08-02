use crate::theme::{CardBack, CardSlot, DividerOrnate, HpBar, OrnateFrame};
use leptos::prelude::*;

#[component]
pub fn ScoundrelPage() -> impl IntoView {
    view! {
        <div class="mx-auto max-w-5xl px-5 py-12">
            <GameHeader />
            <DividerOrnate />
            <Board />
            <DividerOrnate />
            <Rules />
        </div>
    }
}

#[component]
fn GameHeader() -> impl IntoView {
    view! {
        <header class="text-center">
            <p class="badge">"Solitaire • Dungeon Crawling"</p>
            <h1 class="mt-6 font-display text-5xl font-bold tracking-[0.14em] text-gold-200 uppercase md:text-6xl">
                "Scoundrel"
            </h1>
            <p class="mt-4 font-display text-base tracking-[0.24em] text-parchment-400 uppercase">
                "Enter the room. Face the monsters. Outlast the deck."
            </p>
        </header>
    }
}

#[component]
fn Board() -> impl IntoView {
    view! {
        <section>
            <OrnateFrame>
                {/* — Status panel: health + turn — */}
                <div class="flex flex-wrap items-center justify-between gap-6">
                    <div class="flex items-center gap-4">
                        <span class="font-display text-sm tracking-[0.2em] text-parchment-300 uppercase">
                            "Health"
                        </span>
                        <div class="w-48">
                            <HpBar percent="100%" />
                        </div>
                        <span class="font-display text-sm text-gold-300">"20/20"</span>
                    </div>
                    <div class="flex items-center gap-6 font-display text-sm tracking-[0.2em] uppercase">
                        <span class="text-parchment-300">
                            "Turn: "
                            <span class="text-gold-300">"1"</span>
                        </span>
                        <span class="text-parchment-300">
                            "Cards: "
                            <span class="text-gold-300">"52"</span>
                        </span>
                    </div>
                </div>

                {/* — Board: deck + room + armory — */}
                <div class="mt-12 grid items-center gap-10 md:grid-cols-[auto_1fr_auto]">
                    <DeckColumn />

                    <div class="order-first md:order-none">
                        <p class="mb-5 text-center font-display text-xs tracking-[0.3em] text-parchment-400 uppercase">
                            "The room"
                        </p>
                        <div class="grid grid-cols-2 gap-5 sm:grid-cols-4">
                            {(0..4)
                                .map(|_i| {
                                    view! { <CardSlot class="h-40 sm:h-48" label="Room" /> }
                                })
                                .collect_view()}
                        </div>
                    </div>

                    <ArmoryColumn />
                </div>

                {/* — Actions — */}
                <div class="mt-16 flex flex-wrap justify-center gap-4">
                    <button class="btn btn-gold" type="button">"New turn"</button>
                    <button class="btn btn-wine" type="button">"Attack"</button>
                    <button class="btn btn-outline" type="button">"Flee"</button>
                    <button class="btn btn-outline" type="button">"Equip"</button>
                </div>
            </OrnateFrame>
        </section>
    }
}

#[component]
fn DeckColumn() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center gap-3">
            <p class="font-display text-xs tracking-[0.3em] text-parchment-400 uppercase">"Deck"</p>
            <CardBack class="h-40 w-28" />
            <span class="mt-1 w-28 text-center font-display text-[0.62rem] leading-relaxed tracking-[0.16em] text-parchment-300 uppercase">
                "52 cards"
            </span>
        </div>
    }
}

#[component]
fn ArmoryColumn() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center gap-3">
            <p class="font-display text-xs tracking-[0.3em] text-parchment-400 uppercase">"Armory"</p>
            <CardSlot class="h-40 w-28" />
            <span class="mt-1 w-28 text-center font-display text-[0.62rem] leading-relaxed tracking-[0.16em] text-parchment-300 uppercase">
                "Equipped weapon"
            </span>
        </div>
    }
}

#[component]
fn Rules() -> impl IntoView {
    view! {
        <section>
            <h2 class="section-title text-center text-2xl">"How to play"</h2>
            <div class="mt-8 grid gap-5 md:grid-cols-3">
                <RuleCard
                    icon="1"
                    title="Draw"
                    text="Each turn you draw 5 cards from the deck: one goes to the armory, the other four to the room."
                />
                <RuleCard
                    icon="2"
                    title="Fight"
                    text="Black cards are weapons to equip, red cards are monsters to face with your blade."
                />
                <RuleCard
                    icon="3"
                    title="Survive"
                    text="You have 20 health points. Outlast all 52 cards to win the game."
                />
            </div>
        </section>
    }
}

#[component]
fn RuleCard(icon: &'static str, title: &'static str, text: &'static str) -> impl IntoView {
    view! {
        <div class="project-card p-6">
            <div class="flex items-center gap-4">
                <span class="grid h-10 w-10 shrink-0 place-items-center rounded-full border border-gold-500/50 bg-wine-900/60 font-display text-gold-300">
                    {icon}
                </span>
                <h3 class="font-display text-lg tracking-[0.12em] text-gold-200 uppercase">{title}</h3>
            </div>
            <p class="mt-4 leading-relaxed text-parchment-200">{text}</p>
        </div>
    }
}

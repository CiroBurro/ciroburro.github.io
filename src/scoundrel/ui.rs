use crate::scoundrel::logic::{Card, GameState, Suit, MAX_HP, ROOM_SIZE};
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

/// Faccia di una carta da gioco, con rank, seme e tipo.
#[component]
fn CardFace(card: Card, #[prop(optional)] selected: bool) -> impl IntoView {
    let rank = match card.value {
        11 => "J".to_string(),
        12 => "Q".to_string(),
        13 => "K".to_string(),
        14 => "A".to_string(),
        v => v.to_string(),
    };
    let suit = match card.suit {
        Suit::Spades => "♠",
        Suit::Clubs => "♣",
        Suit::Diamonds => "♦",
        Suit::Hearts => "♥",
    };
    let kind = if card.is_monster() {
        "Monster"
    } else if card.is_weapon() {
        "Weapon"
    } else {
        "Potion"
    };
    let red = card.is_weapon() || card.is_potion();
    let color = if red { "text-wine-600" } else { "text-ink-900" };

    view! {
        <div
            class="card-face relative flex h-40 w-28 flex-col justify-between p-2 transition-transform duration-200"
            class:scale-105=selected
        >
            <div class="flex items-start justify-between">
                <span class=format!("font-display text-lg leading-none {color}")>
                    {rank.clone()}
                </span>
                <span class="rounded bg-ink-900/80 px-1 py-0.5 font-display text-[0.5rem] tracking-[0.14em] text-parchment-200 uppercase">
                    {kind}
                </span>
            </div>
            <span class=format!("suit text-center {color}")>
                {suit}
            </span>
            <span class=format!("self-end font-display text-lg leading-none rotate-180 {color}")>
                {rank}
            </span>
        </div>
    }
}

#[component]
fn Board() -> impl IntoView {
    let state = RwSignal::new(GameState::new());
    let selected = RwSignal::new(None::<Card>);
    let notice = RwSignal::new(String::new());

    let game_over = move || state.get().is_dead() || state.get().has_won();
    let hp_pct = Signal::derive(move || {
        (state.get().hp.max(0) as f64 / MAX_HP as f64) * 100.0
    });

    let on_select = move |card: Card| {
        selected.set(Some(card));
        notice.set(String::new());
    };

    let on_attack = move |_| {
        match selected.get_untracked() {
            Some(card) => match state.try_update(|s| s.attack(card)) {
                Some(Ok(())) => notice.set(String::new()),
                Some(Err(e)) => notice.set(e),
                None => notice.set("Game over".to_string()),
            },
            None => notice.set("Select a monster card first".to_string()),
        }
        selected.set(None);
    };

    let on_equip = move |_| {
        match selected.get_untracked() {
            Some(card) => match state.try_update(|s| s.equip(card)) {
                Some(Ok(())) => notice.set(String::new()),
                Some(Err(e)) => notice.set(e),
                None => notice.set("Game over".to_string()),
            },
            None => notice.set("Select a weapon or potion card first".to_string()),
        }
        selected.set(None);
    };

    let on_flee = move |_| {
        match state.try_update(|s| s.flee()) {
            Some(Ok(())) => notice.set(String::new()),
            Some(Err(e)) => notice.set(e),
            None => notice.set("Game over".to_string()),
        }
        selected.set(None);
    };

    let on_new_turn = move |_| {
        match state.try_update(|s| s.new_turn()) {
            Some(Ok(())) => notice.set(String::new()),
            Some(Err(e)) => notice.set(e),
            None => notice.set("Game over".to_string()),
        }
        selected.set(None);
    };

    let restart = move |_: leptos::ev::MouseEvent| {
        state.set(GameState::new());
        selected.set(None);
        notice.set(String::new());
    };

    view! {
        <section>
            <OrnateFrame>
                {/* — Status panel: health + turn + deck — */}
                <div class="flex flex-wrap items-center justify-between gap-6">
                    <div class="flex items-center gap-4">
                        <span class="font-display text-sm tracking-[0.2em] text-parchment-300 uppercase">
                            "Health"
                        </span>
                        <div class="w-48">
                            <HpBar percent=hp_pct />
                        </div>
                        <span class="font-display text-sm text-gold-300">
                            {move || format!("{}/{}", state.get().hp.max(0), MAX_HP)}
                        </span>
                    </div>
                    <div class="flex items-center gap-6 font-display text-sm tracking-[0.2em] uppercase">
                        <span class="text-parchment-300">
                            "Turn: "
                            <span class="text-gold-300">{move || state.get().turn}</span>
                        </span>
                        <span class="text-parchment-300">
                            "Cards: "
                            <span class="text-gold-300">{move || state.get().dungeon_deck.len()}</span>
                        </span>
                    </div>
                </div>

                {/* — Verdict (win / death) — */}
                {move || {
                    let s = state.get();
                    if s.is_dead() {
                        view! {
                            <div class="mt-6 rounded-lg border border-wine-500/60 bg-wine-900/50 px-4 py-4 text-center">
                                <p class="font-display text-sm tracking-[0.2em] text-wine-200 uppercase">
                                    "You died in the dungeon…"
                                </p>
                                <button class="btn btn-gold mt-3" type="button" on:click=restart>
                                    "Play again"
                                </button>
                            </div>
                        }
                        .into_any()
                    } else if s.has_won() {
                        view! {
                            <div class="mt-6 rounded-lg border border-gold-500/60 bg-gold-900/30 px-4 py-4 text-center">
                                <p class="font-display text-sm tracking-[0.2em] text-gold-200 uppercase">
                                    {format!("You cleared the dungeon! Score: {}", s.hp.max(0))}
                                </p>
                                <button class="btn btn-gold mt-3" type="button" on:click=restart>
                                    "Play again"
                                </button>
                            </div>
                        }
                        .into_any()
                    } else {
                        view! {}.into_any()
                    }
                }}

                {/* — Board: deck + room + armory — */}
                <div class="mt-12 grid items-center gap-10 md:grid-cols-[auto_1fr_auto]">
                    {/* Deck */}
                    <div class="flex flex-col items-center gap-3">
                        <p class="font-display text-xs tracking-[0.3em] text-parchment-400 uppercase">"Deck"</p>
                        <CardBack class="h-40 w-28" />
                        <span class="mt-1 w-28 text-center font-display text-[0.62rem] leading-relaxed tracking-[0.16em] text-parchment-300 uppercase">
                            {move || format!("{} cards", state.get().dungeon_deck.len())}
                        </span>
                    </div>

                    {/* Room */}
                    <div class="order-first md:order-none">
                        <p class="mb-5 text-center font-display text-xs tracking-[0.3em] text-parchment-400 uppercase">
                            "The room"
                        </p>
                        <div class="grid grid-cols-2 gap-5 sm:grid-cols-4">
                            {(0..ROOM_SIZE)
                                .map(|i| {
                                    let card = move || state.get().current_room[i];
                                    let is_selected = move || selected.get() == card();
                                    view! {
                                        {move || match card() {
                                            Some(c) => view! {
                                                <div
                                                    class="cursor-pointer"
                                                    class:scale-105=is_selected
                                                    on:click=move |_| on_select(c)
                                                >
                                                    <CardFace card=c selected=is_selected() />
                                                </div>
                                            }
                                            .into_any(),
                                            None => view! {
                                                <CardSlot class="h-40 sm:h-48" label="Room" />
                                            }
                                            .into_any(),
                                        }}
                                    }
                                })
                                .collect_view()}
                        </div>
                    </div>

                    {/* Armory */}
                    <div class="flex flex-col items-center gap-3">
                        <p class="font-display text-xs tracking-[0.3em] text-parchment-400 uppercase">"Armory"</p>
                        {move || match state.get().weapon {
                            Some(w) => view! {
                                <div class="transition-transform duration-200">
                                    <CardFace card=Card::new(Suit::Diamonds, w.value) />
                                </div>
                            }
                            .into_any(),
                            None => view! {
                                <CardSlot class="h-40 w-28" />
                            }
                            .into_any(),
                        }}
                        <span class="mt-1 w-28 text-center font-display text-[0.62rem] leading-relaxed tracking-[0.16em] text-parchment-300 uppercase">
                            {move || if state.get().weapon.is_some() { "Equipped weapon" } else { "No weapon" }}
                        </span>
                    </div>
                </div>

                {/* — Notice — */}
                {move || {
                    if notice.get().is_empty() {
                        view! {}.into_any()
                    } else {
                        view! {
                            <p class="mt-8 text-center text-sm text-wine-300">
                                {notice.get()}
                            </p>
                        }
                        .into_any()
                    }
                }}

                {/* — Actions — */}
                <div class="mt-8 flex flex-wrap justify-center gap-4">
                    <button class="btn btn-gold" type="button" disabled=game_over on:click=on_new_turn>
                        "New turn"
                    </button>
                    <button class="btn btn-wine" type="button" disabled=game_over on:click=on_attack>
                        "Attack"
                    </button>
                    <button class="btn btn-outline" type="button" disabled=game_over on:click=on_flee>
                        "Flee"
                    </button>
                    <button class="btn btn-outline" type="button" disabled=game_over on:click=on_equip>
                        "Equip"
                    </button>
                </div>
            </OrnateFrame>
        </section>
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
                    text="Each turn you draw 5 cards from the deck: one goes to the armory, the other four to the room. Resolve three, then leave one behind for the next room."
                />
                <RuleCard
                    icon="2"
                    title="Fight"
                    text="Black cards are monsters to fight. Diamonds are weapons to equip, hearts are potions to drink. A weapon reduces the damage you take."
                />
                <RuleCard
                    icon="3"
                    title="Survive"
                    text="You have 20 health points. Outlast the 44-card dungeon deck to win the game."
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

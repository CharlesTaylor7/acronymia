use super::context::*;
use crate::components::state::*;
use crate::components::styles::*;
use crate::types::ClientMessage::*;
use ::leptos::*;

#[component]
pub fn GameSetup() -> impl IntoView {
    let is_host = use_typed_context::<Memo_IsHost>();
    let player_name = use_typed_context::<Signal_PlayerName>();

    let players = use_typed_context::<Memo_Players>();
    let game_state = use_typed_context::<Signal_GameState>();

    let start_game_action = create_ws_action();
    let start_game = move || {
        let config = StartGame(game_state.with(|g| g.config.clone()));
        start_game_action.dispatch(config);
    };

    let join_game_action = create_ws_action();
    let join_game = move || {
        join_game_action.dispatch(JoinGame {
            name: player_name.get(),
        });
    };

    view! {
        <label>"Pick a Nickname to join: "</label>
        <input
            type="text"
            class=text_input_class("")
            value=player_name
            on:input=move |e| player_name.set(event_target_value(&e))
            on:keydown=move |e| if e.key() == "Enter" { join_game(); }
        />
        <div class="flex flex-row gap-4">
            <button
                class=ButtonStyle::Primary.class()
                on:click=move|_| join_game()
            >
            {move|| if join_game_action.version().get() > 0 { "Update name" } else { "Join" }}
            </button>
            <Show when=is_host fallback=|| ()>
                <button
                    class=ButtonStyle::Secondary.class()
                    disabled=move|| players.with(|ps| ps.len() < 3)
                    on:click=move|_| start_game()
                >
                    "Start game"
                </button>
            </Show>
        </div>
        <div>
            <p>{move || players.with(|ps| ps.len())}" players joined"</p>
            <ul class="list-inside list-disc flex flex-col items-start">
                <For
                    each=players
                    key=|p| format!("{}-{}", p.id, p.name)
                    view=|p| view! { <li>{p.name}</li> }
                />
            </ul>
        </div>
        <h1 class="text-xl font-bold">"Configuration"</h1>
        <ConfigureAcronymLength />
    }
}

#[component]
pub fn ConfigureAcronymLength() -> impl IntoView {
    const MIN_ACRONYM_LENGTH: usize = 2;

    let g = use_typed_context::<Signal_GameState>();
    let (min, set_min) = create_slice(
        g,
        move |g| g.config.letters_per_acronym.min,
        move |g, v| g.config.letters_per_acronym.min = v,
    );
    let (max, set_max) = create_slice(
        g,
        move |g| g.config.letters_per_acronym.max,
        move |g, v| g.config.letters_per_acronym.max = v,
    );

    view! {
        <div class="flex flex-row gap-2 items-start">
            "From"
            <input
                type="number"
                class=number_input_class("w-[4rem]")
                min=MIN_ACRONYM_LENGTH
                prop:max=max
                prop:value=min
                on:change=move|e| {
                    if let Ok(n) = event_target_value(&e).parse() {
                        set_min.set(n);
                    }
                }
            />
            "to"
            <input
                type="number"
                class=number_input_class("w-[4rem]")
                prop:min=min
                prop:value=max
                on:input=move|e| {
                    if let Ok(n) = event_target_value(&e).parse() {
                        set_max.set(n);
                    }
                }
            />
            "letters per acronym"
        </div>
    }
}

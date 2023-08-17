mod gui;
mod game;
mod storage;

use yew::prelude::*;
use gloo_console::log;

use gui::*;
use game::*;
use storage::*;

const BOARD_SIZE: u32 = 10;

#[function_component]
fn App() -> Html {
    // State Handlers/Data used in the game.
    let current_ship = use_state_eq(|| Ship::new("destroyer", 5));

    let hit_to_place = use_state_eq(|| (0u32, 0u32));
    let hit_active = use_state_eq(|| false);

    let placed_hits = use_state_eq(|| Vec::<Vector2>::new());
    let ships = use_state_eq(|| create_ships(BOARD_SIZE));

    let game_finished = use_state_eq(|| false);

    // Resets game
    let reset_game = {
        // Required state handlers
        let placed_hits = placed_hits.clone();
        let ships = ships.clone();
        let game_finished = game_finished.clone();

        // Reset state handlers through callback
        Callback::from(move |_| {
            ships.set(create_ships(BOARD_SIZE));
            placed_hits.set(vec![]);
            game_finished.set(false);
        })
    };

    // Status of cell/button
    let cell_status: Callback<Vector2, CellStatus> = {
        // Required state handlers
        let placed_hits = placed_hits.clone();
        let ships = ships.clone();

        // Callback
        Callback::from(move |pos: Vector2| {
            // Loops through hit list
            for hit in &*placed_hits {
                if *hit != pos { continue; }

                return match position_hits_ship(&ships, *hit) {
                    true => CellStatus::Hit,
                    false => CellStatus::Miss,
                }
            }

            CellStatus::None
        })
    };

    let hit_place = {
        // Required state handlers
        let placed_hits = placed_hits.clone();
        let game_finished = game_finished.clone();
        let ships = ships.clone();

        // Actual Callback
        Callback::from(move |pos: Vector2| {
            // Pushes hit onto hits list
            let mut hits = (*placed_hits).clone();
            hits.push(pos);
            placed_hits.set(hits.clone());
            storage::get_high_score();

            // Win Condition
            if game::all_ships_sunk(&ships, &hits) { 
                game_finished.set(true); 
                storage::set_high_score(hits.len());
            }
        })
    };

    html! {
        <>
        <div class="game-container">

        // Menu
        <div class="button-menu">
            <button class="menu-button" style="text-align: left;">{"←  Back"}</button>
            <div style="flex: .1 1 0;"/>
            <button class="menu-button" style="text-align: right;">{"Retry  →"}</button>
        </div>

        // Main Board
        <BoardGUI click = {hit_place} {cell_status} 
        board_size = {BOARD_SIZE} active = {game_finished.clone()}/>

        // Win Notification
        <Notification active={game_finished.clone()} {reset_game}
        right_button={"← Close"} left_button={"Retry →"}>

            <center>{"You Won!"}</center>
            {format!("You placed {:?} hits", placed_hits.len())}
            <br/>
            {format!("Your best has been {} hits", {storage::get_high_score()})}

        </Notification>
        </div>
        </>
    }
}

// Game runner
fn main() {
    yew::Renderer::<App>::new().render();
}
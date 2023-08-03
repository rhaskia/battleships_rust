mod gui;
mod game;

use yew::prelude::*;
use gloo_console::log;
use wasm_bindgen::JsValue;

use gui::*;
use game::*;

#[function_component]
fn App() -> Html
{
    let current_ship = use_state(|| Ship::new("destroyer", 5));

    let hit_to_place = use_state(|| (0u32, 0u32));
    let hit_active = use_state(|| false);

    let mut placed_hits = use_state(|| Vec::<Vector2>::new());
    let ships = create_ships();

    let length: Callback<(), Vector2> = {
        let current_ship = current_ship.clone();
        Callback::from(move |_| {
        current_ship.size()
    })
    };

    let cell_status: Callback<Vector2, CellStatus> = {
        let placed_hits = placed_hits.clone();
        Callback::from(move |pos: Vector2| {
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

    let hit_hover = {
        let hit_to_place = hit_to_place.clone();
        let hit_active = hit_active.clone();

        Callback::from(move |pos: Vector2| {
            hit_to_place.set(pos);
            hit_active.set(true);
        })
    };

    let hit_place = {
        let placed_hits = placed_hits.clone();
        Callback::from(move |pos: Vector2| {
            let mut hits = (*placed_hits).clone();
            hits.push(pos);
            log!(hits.len());
            placed_hits.set(hits);
        })
    };
    
    let ship_control = {
        let current_ship = current_ship.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key_code() == 'R' as u8 as u32
            {
                current_ship.set(current_ship.rotate());
                log!(current_ship.is_vertical());
            }
        })
    };

    html! 
    {
        <>
        <div class="game-container">
        <BoardGUI click={hit_place} {cell_status}
        keydown={ship_control.clone()}/>
        </div>

        <CurrentHitGUI position={hit_to_place.clone()}/>
        </>
    }
}

fn main() 
{
    yew::Renderer::<App>::new().render();
}
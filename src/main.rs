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

    let ship_pos = use_state(|| (0u32, 0u32));
    let ship_active = use_state(|| false);

    let mut placed_ships = use_state(|| Vec::<Ship>::new());
    let ships_cb: Callback<(), Vec<(Vector2, Vector2)>> = {
        let placed_ships = placed_ships.clone();
        Callback::from(move |_| {
            placed_ships.iter().map(|ship| (ship.get_position(), ship.size()))
                .collect::<Vec<(Vector2, Vector2)>>()
        })
    };

    let length: Callback<(), Vector2> = {
        let current_ship = current_ship.clone();
        Callback::from(move |_| {
        current_ship.size()
    })
    };

    let ship_hover = {
        let ship_pos = ship_pos.clone();
        let ship_active = ship_active.clone();

        Callback::from(move |pos: Vector2| {
            ship_pos.set(pos);
            ship_active.set(pos.0 + 5 <= 10);
        })
    };

    let ship_place = {
        let placed_ships = placed_ships.clone();
        Callback::from(move |pos: Vector2| {
            let mut ships = (*placed_ships).clone();
            ships.push(Ship::new("whuh", 3));
            placed_ships.set(ships);
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
        <BoardGUI hover={ship_hover} click={ship_place} 
        keydown={ship_control.clone()} active={ship_active.clone()}/>

        <CurrentShipGUI position={ship_pos.clone()}
         active={ship_active.clone()} {length}/>

        <ShipsGUI ships={ships_cb} />
        </>
    }
}

fn main() 
{
    yew::Renderer::<App>::new().render();
}
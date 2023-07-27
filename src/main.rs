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
    let currentShip = use_state(|| Ship::new("destroyer", 5));

    let ship_pos = use_state(|| (0u32, 0u32));
    let ship_active = use_state(|| false);

    let mut placed_ships = use_state(|| Vec::<Ship>::new());

    let length: Callback<(), Vector2> = {
        let currentShip = currentShip.clone();
        Callback::from(move |_| {
        log!(currentShip.size().0);
        currentShip.size()
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
            placed_ships.set(vec![]);
        })
    };
    
    let ship_control = {
        let currentShip = currentShip.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key_code() == 'R' as u8 as u32
            {
                currentShip.set(currentShip.rotate());
                log!(currentShip.is_vertical());
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
        </>
    }
}

fn main() 
{
    yew::Renderer::<App>::new().render();
}
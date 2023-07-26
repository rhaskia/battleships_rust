mod gui;
mod game;

use yew::prelude::*;
use gui::*;
use game::*;
use gloo_console::log;
use wasm_bindgen::JsValue;

#[function_component]
fn App() -> Html
{
    let currentShip = game::ship::Ship::new("destroyer", 5, (0, 0), false);

    let ship_pos = use_state(|| (0u32, 0u32));
    let ship_active = use_state(|| false);
    let length = currentShip.size();

    let ship_hover = {
        let ship_pos = ship_pos.clone();
        let ship_active = ship_active.clone();

        Callback::from(move |pos: Vector2| {
            ship_pos.set(pos);
            ship_active.set(pos.0 + 5 <= 10);
        })
    };

    let ship_place = {
        Callback::from(move |pos: Vector2| {
            log!("Hello", JsValue::from(pos.0), JsValue::from(pos.1))
        })
    };
    
    let ship_control = {
        Callback::from(move |e: KeyboardEvent| {
            log!("Hello", (e.key_code() as u8 as char).to_string())
        })
    };

    html! 
    {
        <>
        <BoardGUI hover={ship_hover} click={ship_place} 
        active={ship_active.clone()} keydown={ship_control}/>
        <CurrentShipGUI position={ship_pos.clone()}
         active={ship_active.clone()} {length}/>
        </>
    }
}

fn main() 
{
    yew::Renderer::<App>::new().render();
}
mod gui;
mod game;

use yew::prelude::*;
use gui::*;
use game::*;

#[function_component]
fn App() -> Html
{
    let currentShip = game::ship::Ship::new("destroyer", 5, (0, 0), true);

    let ship_pos = use_state(|| (0u32, 0u32));
    let ship_active = use_state(|| false);
    let length = currentShip.length;

    let mouseover = {
        let ship_pos = ship_pos.clone();
        let ship_active = ship_active.clone();
        Callback::from(move |pos: Vector2| {
            ship_pos.set(pos);
            ship_active.set(pos.0 + 5 <= 10);
        })
    };

    html! 
    {
        <>
        <BoardGUI callback={mouseover} active={ship_active.clone()}/>
        <CurrentShipGUI position={ship_pos.clone()}
         active={ship_active.clone()} {length}/>
        </>
    }
}

fn main() 
{
    yew::Renderer::<App>::new().render();
}
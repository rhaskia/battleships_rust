use yew::prelude::*;
use yew::virtual_dom::ListenerKind::onkeydown;

pub type Vector2 = (u32, u32);

#[derive(PartialEq)]
pub enum CellStatus {
    None,
    Hit,
    Miss,
}

#[derive(Properties, PartialEq)]
pub struct BoardProps {
    pub click: Callback<Vector2>,
    pub keydown: Callback<KeyboardEvent>,
    pub cell_status: Callback<Vector2, CellStatus>
}

#[function_component]
pub fn BoardGUI(props: &BoardProps) -> Html {
    let click = |x: u32, y: u32| {
        let ship_click = props.click.clone();
        Callback::from(move |_| ship_click.emit((x, y)))
    };

    let keydown = {
        let ship_key = props.keydown.clone();
        Callback::from(move |e: KeyboardEvent| ship_key.emit(e))
    };

    let cs = |x: u32, y: u32| -> &str {
        match props.cell_status.clone().emit((x, y)) {
            CellStatus::None => "",
            CellStatus::Hit => "hit",
            CellStatus::Miss => "miss",
        }
    };

    let inner = |x: u32, y: u32| -> Html {
        match props.cell_status.clone().emit((x, y)) {
            CellStatus::None => html!{<></>},
            CellStatus::Hit => html!{
                <svg class="hit-marker" viewBox="0 0 20 20" fill="none" xmlns="http://www.w3.org/2000/svg" alt="hit">
                <path d="M2 2.00049L10 10.0005M18 18.0005L10 10.0005M10 10.0005L18 2.00049M10 10.0005L2 18.0005" stroke="black" stroke-width="2.5" stroke-linecap="round"/>
                </svg>},
            CellStatus::Miss => html!{
                <svg class="miss-marker" viewBox="0 0 10 10" fill="none" xmlns="http://www.w3.org/2000/svg" alt="miss">
                <circle cx="5" cy="5.00049" r="5" fill="black"/>
                </svg>},
        }
    };

    html! {
        <div class="grid-container">
        <div class="grid">
            <div class="grid-row" style="max-height: 5%;">
            <div class="grid-label" style="min-width: 2%;"></div>
            { (0..10).map(|y|
            html!{
                <div class="grid-number grid-label">{(y + 1)}</div>
            }).collect::<Html>()}
            </div>

            { (0..10).map(|y| html! {
            <>
            <div class="grid-row">
                <div class="grid-letter grid-label">{(y + 65) as u8 as char}</div>
                {(0..10).map(|x| 
                html! {

                <button
                class ={format!("grid-button {}", cs(x, y))}
                title = {format!("{}{}", (y + 65) as u8 as char, x)}
                disabled = {cs(x, y) != ""}
                onmousedown = {click(x, y)}
                onkeydown = {keydown.clone()}>
                {inner(x, y)}
                </button>

                }).collect::<Html>()}
            </div>
            </>
            }).collect::<Html>()}
        </div>

        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct CShipProps {
    pub position: UseStateHandle<Vector2>,
}

#[function_component]
pub fn CurrentHitGUI(props: &CShipProps) -> Html
{
    let (x, y) = &*props.position;

    html! {
        <div class="ship" style={format!("left: {}; top: {}; 
        width: 30px; height: 30px;",
        (x*30) + 38, (y*30) + 38)}/>
    }
}

#[derive(Properties, PartialEq)]
pub struct ShipsProps {
    pub ships: Callback<(), Vec<(Vector2, Vector2)>>,
}

#[function_component]
pub fn ShipsGUI(props: &ShipsProps) -> Html
{
    let ships = props.ships.emit(());

    html! {
        <div>
            {ships.iter().map(
            |ship| html!{<div style="display:block; width: 30px; height: 40px"/>}
            ).collect::<Html>()}
        </div>
    }
}
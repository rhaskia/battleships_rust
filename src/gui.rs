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
    pub active: UseStateHandle<bool>,
    pub hover: Callback<Vector2>,
    pub click: Callback<Vector2>,
    pub keydown: Callback<KeyboardEvent>,
    pub cell_status: Callback<Vector2, CellStatus>
}

#[function_component]
pub fn BoardGUI(props: &BoardProps) -> Html {
    let cb = {
        let active = props.active.clone();
        Callback::from(move |_| active.set(false))
    };

    let hover = |x: u32, y: u32| {
        let ship_hover = props.hover.clone();
        Callback::from(move |_| ship_hover.emit((x, y)))
    };

    let click = |x: u32, y: u32| {
        let ship_click = props.click.clone();
        Callback::from(move |_| ship_click.emit((x, y)))
    };

    let keydown = {
        let ship_key = props.keydown.clone();
        Callback::from(move |e: KeyboardEvent| ship_key.emit(e))
    };

    html! {
        <div class="gridcontainer">

        <div class="gridrow">
            <div class = "gridnumber"/>
            {(0..10).map(|x| html!
            { <div class="gridnumber"> { format!("{}", x + 1)} </div> })
            .collect::<Html>()}
        </div>

        <div class="grid">
            { (0..10).map(|y|
            html!{ 
            <div class="gridrow">
                <div class="gridnumber">{ format!("{}", y + 1)} </div>
                {(0..10).map(|x| 
                html! {
                <button class="gridbutton"
                disabled = {props.cell_status.clone().emit((x, y)) == CellStatus::None}
                onmouseover = {hover(x, y)}
                onmousedown = {click(x, y)}
                onmouseleave={cb.clone()}
                onkeydown={keydown.clone()}>
                </button>
                }).collect::<Html>()}
            </div>
            }).collect::<Html>()}
        </div>

        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct CShipProps {
    pub position: UseStateHandle<Vector2>,
    pub active: UseStateHandle<bool>,
}

#[function_component]
pub fn CurrentHitGUI(props: &CShipProps) -> Html
{
    let (x, y) = &*props.position;
    let active = &*props.active;

    html! {
        <div class="ship" style={format!("left: {}; top: {}; 
        width: 30px; height: 30px; display: {};",
        (x*30) + 38, (y*30) + 38,
        if *active {"grid"} else {"none"})}/>
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
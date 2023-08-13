use yew::prelude::*;

type Vector2 = (u32, u32);

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
    pub cell_status: Callback<Vector2, CellStatus>,
    pub active: UseStateHandle<bool>,
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
                <svg class="miss-marker" viewBox="0 0 20 20" fill="none" xmlns="http://www.w3.org/2000/svg" alt="miss">
                <circle cx="10" cy="10" r="5" fill="black"/>
                </svg>},
        }
    };

    html! {
        <>
        <div class="grid-container">
        <div class="grid">
            <div class="grid-row" style="max-height: 5%;">
            <div class="grid-label" style="min-width: 5%;"></div>
            { (0..10).map(|y|
            html!{
                <div class="grid-number grid-label">{y + 1}</div>
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
                title = {format!("{}{}", (y + 65) as u8 as char, x + 1)}
                disabled = {*props.active || cs(x, y) != ""}
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
        </>
    }
}

#[derive(Properties, PartialEq)]
pub struct NoticeProps {
    pub active: UseStateHandle<bool>,
    pub left_button: &'static str,
    pub right_button: &'static str,
    pub reset_game: Callback<MouseEvent>,
    pub children: Children,
}

#[function_component]
pub fn Notification(props: &NoticeProps) -> Html
{
    let active = *props.active;

    html! {
        <div class="notification" 
        style={format!("display: {};", if active {"flex"} else {"none"})}>
            {props.children.clone()}

            <div style="flex: 1;"/>

            <div style="width: 100%; display: flex; align-self: flex-end;">

            <button class="menu-button" style="text-align: left; height: 100%;"> 
            {props.right_button} </button>

            <div style="flex: .1 1 0;"/>
            
            <button class="menu-button" style="text-align: right; height: 100%;" onclick={props.reset_game.clone()}>
            {props.left_button} </button>

            </div>
        </div>
    }
}
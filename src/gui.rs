use yew::prelude::*;

pub type Vector2 = (u32, u32);

#[derive(Properties, PartialEq)]
pub struct BoardProps {
    pub active: UseStateHandle<bool>,
    pub callback: Callback<Vector2>
}

#[function_component]
pub fn BoardGUI(props: &BoardProps) -> Html {
    let cb = {
        let active = props.active.clone();
        Callback::from(move |_| active.set(false))
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
                onmouseover = {
                    let callback = props.callback.clone();
                    Callback::from(
                        move |_| callback.emit((x, y)))
                }
                onmouseleave={cb.clone()}>
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
    pub length: u32,
}

#[function_component]
pub fn CurrentShipGUI(props: &CShipProps) -> Html
{
    let (x, y) = &*props.position;
    let active = &*props.active;
    let length = props.length;

    html! {
        <div class="ship" style={format!("left: {}; top: {}; width: {}; display: {};", 
        (x*30) + 38, (y*30) + 38, length * 30, if *active {"grid"} else {"none"})}/>
    }
}
use yew::prelude::*;

#[function_component]
pub fn BoardGUI() -> Html {
    html! {
        <div class="gridcontainer">

        <div class="gridrow">
            <div class = "gridnumber"/>
            {(0..10).map(|x| html!
            { <div class="gridnumber">{format!("{}", x + 1)}</div> })
            .collect::<Html>()}
        </div>

        <div class="grid">
            { (0..10).map(|x|
            html!{ 
            <div class="gridrow">
                <div class="gridnumber">{format!("{}", x + 1)}</div>
                {(0..10).map(|y| html!
                {<button class="gridbutton"></button>})
                .collect::<Html>()}
            </div>
            }).collect::<Html>()}
        </div>

        </div>
    }
}
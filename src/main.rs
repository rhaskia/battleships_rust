use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <div class="gridcontainer">
            { (0..10).map(|_|
            html!{ 
            <div class="gridrow">
                {(0..10).map(|x| html!{<button class="gridbutton"></button>}).collect::<Html>()}
            </div>
            }).collect::<Html>()}
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
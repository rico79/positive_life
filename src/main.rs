use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <div>
            <p>{ "Coucou :) !!!" }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

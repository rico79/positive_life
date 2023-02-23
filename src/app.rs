use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <h1>{ "Hello World!" }</h1>
            <span>{ "from Rico79 !!! " }</span>
        </main>
    }
}
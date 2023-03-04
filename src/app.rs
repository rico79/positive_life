mod static_pages;
mod router;

use yew::prelude::*;

use router::AppRouter;

// Parent component of the entire application
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <AppRouter />
    }
}
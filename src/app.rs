mod static_pages;
mod navigation;

use yew::prelude::*;

use navigation::AppRouter;

// Parent component of the entire application
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <AppRouter />
    }
}

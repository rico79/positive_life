mod static_pages;

use yew::prelude::*;
use yew_router::prelude::*;

// Main router of the application
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

// Routes definition
fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => crate::app::static_pages::welcome_html(),
        Route::NotFound => crate::app::static_pages::url_not_found_html(),
    }
}

// Parent component of the entire application
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <HashRouter>
            <Switch<Route> render={switch} />
        </HashRouter>
    }
}

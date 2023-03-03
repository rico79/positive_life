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

// App navigation bar component
#[function_component]
fn AppNavigationBar() -> Html {
    html! {
        <div>
            <Link<Route> to={Route::Home} >
                { "| Home |" }
            </Link<Route>>
            <Link<Route> to={Route::NotFound} >
                { "| 404 |" }
            </Link<Route>>
        </div>
    }
}

// App main router component
#[function_component]
pub fn AppRouter() -> Html {
    html! {
        <HashRouter>
            <AppNavigationBar />
            <Switch<Route> render={switch} />
        </HashRouter>
    }
}
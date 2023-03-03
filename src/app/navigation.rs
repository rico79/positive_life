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
        <nav class="mx-auto flex max-w-7xl items-center justify-between p-6">
            <div class="flex lg:flex-1">
                <Link<Route> to={Route::Home} >
                    <img class="h-8 w-auto" src="./icons/lotus-150.png" />
                </Link<Route>>
            </div>
            <div class="flex flex-1 justify-end">
                <a href="#" class="text-sm font-semibold text-gray-900">{ "Profile" }</a>
            </div>
        </nav>
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
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
fn AppNavigationHeader() -> Html {
    html! {
        <nav class="mx-auto flex max-w-7xl items-center justify-between p-6">
            <div class="flex flex-1">
                <Link<Route> to={Route::Home} classes={"m-sm p-sm"} >
                    <img class="h-8 w-auto" src="./icons/lotus-150.png" />
                </Link<Route>>
            </div>
            <div class="flex flex-1 justify-end">
                <Link<Route> to={Route::NotFound} classes={"m-sm p-sm"} >
                    <img class="h-8 w-auto rounded-full" src="https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80" />
                </Link<Route>>
            </div>
        </nav>
    }
}

// App main router component
#[function_component]
pub fn AppRouter() -> Html {
    html! {
        <HashRouter>
            <AppNavigationHeader />
            <Switch<Route> render={switch} />
        </HashRouter>
    }
}
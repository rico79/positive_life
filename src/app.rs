use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{ "Home" }</h1> },
        Route::NotFound => html! {
            <main class="grid min-h-full place-items-center bg-white py-24 px-6 sm:py-32 lg:px-8">
                <div class="text-center">
                    <p class="text-base font-semibold text-indigo-600">
                        { "404" }
                    </p>
                    <h1 class="mt-4 text-3xl font-bold tracking-tight text-gray-900 sm:text-5xl">
                        { "Page not found" }
                    </h1>
                    <p class="mt-6 text-base leading-7 text-gray-600">
                        { "Sorry, we couldn’t find the page you’re looking for." }
                    </p>
                    <div class="mt-10 flex items-center justify-center gap-x-6">
                        <Link<Route> to={Route::Home} classes={"rounded-md bg-indigo-600 px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"}>
                            { "Go back home" }
                        </Link<Route>>
                    </div>
                </div>
            </main>
        },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <HashRouter>
            <Switch<Route> render={switch} />
        </HashRouter>
    }
}

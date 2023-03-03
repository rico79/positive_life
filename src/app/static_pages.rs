use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::navigation::Route;

// Static html for the walecome page
pub fn welcome_html() -> Html {
    html! { 
        <h1>{ "Welcome !!" }</h1>
    }
}

// Static html in case of 404 return with page not found
pub fn url_not_found_html() -> Html {
    html! {
        <main class="grid min-h-full place-items-center bg-white py-24 px-6 sm:py-32 lg:px-8">
            <div class="text-center">
                <p class="text-base font-semibold text-green-400">
                    { "404" }
                </p>
                <h1 class="mt-4 text-3xl font-bold tracking-tight text-gray-900 sm:text-5xl">
                    { "Page not found" }
                </h1>
                <p class="mt-6 text-base leading-7 text-gray-600">
                    { "Sorry, we couldn’t find the page you’re looking for." }
                </p>
                <div class="mt-10 flex items-center justify-center gap-x-6">
                    <Link<Route> to={Route::Home} classes={"rounded-md bg-green-400 px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-green-300 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-green-400"}>
                        { "Go back home" }
                    </Link<Route>>
                </div>
            </div>
        </main>
    }
}
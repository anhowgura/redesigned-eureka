mod components;
mod routes;
use yew::prelude::*;
use routes::router::Browserfunc;
#[function_component]
pub fn App() -> Html {
    html! {
        <>
<Browserfunc/>
        </>
    }
}
use yew::prelude::*;
use yew_router::prelude::*;
use crate::{routes::search::*,components::medium::web3::Web3Login};
use crate::components::medium::login::Login;
use crate::components::medium::createaccount::CreateAccount;
use super::search::{SearchRoute,switch_search};
#[derive(Clone, Routable, PartialEq)]
pub enum MainRoute {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/web3_sign_in")]
    Web3Login,
    #[at("/createaccount")]
    CreateAccount,
    #[at("/{search}")]
    SearchRoot,
    #[at("/search/")]
    Search,
    #[at("/settings")]
    SettingsRoot,
    #[at("/settings/")]
    Settings,
    #[not_found]
    #[at("/404")]
    NotFound
}
#[derive(Clone, Routable, PartialEq)]
enum SettingsRoute {
    #[at("/settings")] //Create an Account for 
    GeneralSettings, 
    #[at("/settings/wallets")]
    Wallets,
    #[at("/settings/mycontracts")]
    MyContracts,
    #[at("/settings/privacy")]
    Privacy,
}


fn switch_main(route: MainRoute) -> Html {
match route {
    MainRoute::Home => html! {},
    MainRoute::Login => html! {<Login/>},
    MainRoute::Web3Login => html! {<Web3Login/>},
    MainRoute::CreateAccount => html! {<CreateAccount/>},
    MainRoute::SearchRoot | MainRoute::Search=> html! {<Switch<SearchRoute> render={switch_search}/>},
    MainRoute::SettingsRoot | MainRoute::Settings=> html! {<Switch<SettingsRoute> render={switch_settings}/>},
    MainRoute::NotFound => html!{},

}
}
fn switch_settings(route: SettingsRoute) -> Html {
    match route {
        SettingsRoute::GeneralSettings => html! (format!("")),
        SettingsRoute::Wallets => {  html! (format!(""))},
        SettingsRoute::MyContracts => html! (format!("")),
        SettingsRoute::Privacy => html! (format!("")),
    }
}


#[function_component]
pub fn Browserfunc() -> Html {
    html! {
        html! {
            <BrowserRouter>
                <Switch<MainRoute> render={switch_main}/>
            </BrowserRouter>
        }
    }
}
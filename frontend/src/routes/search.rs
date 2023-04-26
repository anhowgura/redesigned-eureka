use yew_router::prelude::*;
use yew::prelude::*;
use super::router::MainRoute;
#[derive(Clone, Routable, PartialEq)]
pub enum SearchRoute {
    #[at("/search")]
    Search,
   #[at("/search/g/{query}")]
   G { query: String},
   #[at("/search/u/{query}")]
   U { query: String},
   #[at("/search/c/{query}")]
   C { query: String},

}

pub fn switch_search(route: SearchRoute) -> Html {
    match route {
        SearchRoute::Search => html! {},
        SearchRoute::G {query} => html! {},
        SearchRoute::U {query} => html! {},
        SearchRoute::C {query: String} => html! {},
      //  SearchRoute::Trending => html! {},
      //  SearchRoute::Hashtag {}=> html! {},
    }
}
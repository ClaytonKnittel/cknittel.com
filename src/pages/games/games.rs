use yew::{function_component, html, use_state_eq, Html};

use crate::pages::games::{game_list::GameList, page::Page, rain_game::RainGame};

#[function_component(GamesPage)]
pub fn games_page() -> Html {
  let game_page_state = use_state_eq(|| Page::Home);

  match *game_page_state {
    Page::Home => html! { <GameList change_page={move |page_id| game_page_state.set(page_id)} /> },
    Page::RainGame => html! { <RainGame /> },
  }
}

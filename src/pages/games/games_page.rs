use yew::{function_component, html, use_state_eq, Html};

use crate::pages::games::{game::GameComponent, game_list::GameList, page::Page};

#[function_component(GamesPage)]
pub fn games_page() -> Html {
  let game_page_state = use_state_eq(|| Page::Home);

  match *game_page_state {
    Page::Home => html! { <GameList change_page={move |page_id| game_page_state.set(page_id)} /> },
    Page::Game(game) => html! { <GameComponent game={game} /> },
  }
}

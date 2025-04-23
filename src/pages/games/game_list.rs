use itertools::Itertools;
use strum::IntoEnumIterator;
use yew::{classes, function_component, html, Callback, Html, Properties};

use crate::{pages::games::game::Game, web_util::is_mobile_device};

use super::page::Page;

#[derive(Properties, PartialEq)]
pub struct Props {
  pub change_page: Callback<Page>,
}

#[function_component(GameList)]
pub fn game_list(props: &Props) -> Html {
  let games = Game::iter()
    .filter(|game| !is_mobile_device() || game.mobile_compatible())
    .map(|game| {
      let change_page = props.change_page.clone();
      html! {
        <div
          class={classes!("game-list-entry")}
          onclick={move |_| change_page.emit(Page::Game(game))}
        >
          { game.title() }
        </div>
      }
    })
    .collect_vec();

  if games.is_empty() {
    return html! {
      <div class={classes!("games")}>
        { "Sorry! It looks like no games are compatible on your device, try from a computer :)" }
      </div>
    };
  }

  html! {
    <div class={classes!("games")}>
      <div class={classes!("game-list-title")}>{ "Games:" }</div>
      { games }
    </div>
  }
}

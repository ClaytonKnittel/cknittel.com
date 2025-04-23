use yew::{classes, function_component, html, Callback, Html, Properties};

use super::page::Page;

#[derive(Properties, PartialEq)]
pub struct Props {
  pub change_page: Callback<Page>,
}

#[function_component(GameList)]
pub fn game_list(props: &Props) -> Html {
  let change_page = props.change_page.clone();

  html! {
    <div class={classes!("games")}>
      <div class={classes!("game-list-title")}>{ "Games:" }</div>
      <div class={classes!("game-list-entry")} onclick={move |_| change_page.emit(Page::RainGame)}>
        { "Rain game" }
      </div>
    </div>
  }
}

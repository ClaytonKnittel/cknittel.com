use yew::{classes, function_component, html, Html};

#[function_component(GamesPage)]
pub fn games_page() -> Html {
  html! {
    <div class={classes!("games")}>
      <div class={classes!("game-list-title")}>{ "Games:" }</div>
      <div class={classes!("game-list-entry")}>{ "Rain game" }</div>
    </div>
  }
}

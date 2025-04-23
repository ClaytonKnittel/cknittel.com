use yew::{classes, function_component, html, Html};

#[function_component(RainGame)]
pub fn rain_game() -> Html {
  html! { <div class={classes!("game-list-entry")}>{ "Rain game" }</div> }
}

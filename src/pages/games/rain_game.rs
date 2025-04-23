use yew::{classes, function_component, html, Html};

#[function_component(RainGame)]
pub fn rain_game() -> Html {
  html! {
    <div class={classes!("game-container")}>
      <iframe class={classes!("game-iframe")} src="/games/rain-game/index.html" title="Rain Game" />
    </div>
  }
}

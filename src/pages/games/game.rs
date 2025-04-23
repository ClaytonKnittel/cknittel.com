use strum::EnumIter;
use yew::{classes, function_component, html, Html, Properties};

#[derive(Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum Game {
  RainGame,
}

impl Game {
  pub const fn mobile_compatible(&self) -> bool {
    match self {
      Game::RainGame => false,
    }
  }

  const fn src(&self) -> &'static str {
    match self {
      Game::RainGame => "/games/rain-game/index.html",
    }
  }

  pub const fn title(&self) -> &'static str {
    match self {
      Game::RainGame => "Rain Game",
    }
  }
}

#[derive(Properties, PartialEq)]
pub struct Props {
  pub game: Game,
}

#[function_component(GameComponent)]
pub fn game_component(props: &Props) -> Html {
  html! {
    <div class={classes!("game-container")}>
      <iframe class={classes!("game-iframe")} src={props.game.src()} title={props.game.title()} />
    </div>
  }
}

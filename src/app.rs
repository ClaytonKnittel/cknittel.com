use yew::{classes, function_component, html, Html};

#[function_component(Header)]
fn header() -> Html {
  html! {
    <div class={classes!("header")}>
      <div class={classes!("title")}>{ "cknittel.com" }</div>
      <div class={classes!("menu")}>
        <div class={classes!("menuitem")}>{"Home"}</div>
        <div class={classes!("menuitem")}>{"Games"}</div>
        <div class={classes!("menuitem")}>{"Projects"}</div>
      </div>
    </div>
  }
}

#[function_component(Body)]
fn body() -> Html {
  html! {
    <div class={classes!("body")}>{ "contents" }</div>
  }
}

#[function_component(App)]
pub fn app() -> Html {
  html! {
    <>
      <Header />
      <Body />
    </>
  }
}

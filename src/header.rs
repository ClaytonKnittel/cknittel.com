use yew::{classes, function_component, html, Callback, Html, Properties};

use crate::page::Page;

#[derive(Properties, PartialEq)]
pub struct Props {
  pub page: Page,
  pub change_page: Callback<Page>,
}

#[function_component(Header)]
pub fn header(props: &Props) -> Html {
  let change_page = props.change_page.clone();
  let to_home_page = move |_| change_page.clone().emit(Page::Home);
  let change_page = props.change_page.clone();
  let to_games_page = move |_| change_page.clone().emit(Page::Games);
  let change_page = props.change_page.clone();
  let to_projects_page = move |_| change_page.clone().emit(Page::Projects);

  html! {
    <div class={classes!("header")}>
      <div class={classes!("title")}>{ "cknittel.com" }</div>
      <div class={classes!("menu")}>
        <div class={classes!("menuitem")} onclick={to_home_page}>{"Home"}</div>
        <div class={classes!("menuitem")} onclick={to_games_page}>{"Games"}</div>
        <div class={classes!("menuitem")} onclick={to_projects_page}>{"Projects"}</div>
      </div>
    </div>
  }
}

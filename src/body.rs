use yew::{classes, function_component, html, Html, Properties};

use crate::{
  page::Page,
  pages::{games::GamesPage, home::HomePage},
};

#[derive(Properties, PartialEq)]
pub struct Props {
  pub page: Page,
}

fn page_contents(page: Page) -> Html {
  match page {
    Page::Home => html! { <HomePage /> },
    Page::Games => html! { <GamesPage /> },
    Page::Projects => html! { "projects" },
  }
}

#[function_component(Body)]
pub fn body(props: &Props) -> Html {
  html! { <div class={classes!("body")}>{ page_contents(props.page) }</div> }
}

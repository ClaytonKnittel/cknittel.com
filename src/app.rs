use yew::{function_component, html, use_state_eq, Html};

use crate::{body::Body, header::Header, page::Page};

#[function_component(App)]
pub fn app() -> Html {
  let page_state = use_state_eq(|| Page::Home);
  let page = page_state.clone();

  html! {
    <>
      <Header page={*page} change_page={move |page_id| page_state.set(page_id)} />
      <Body page={*page} />
    </>
  }
}

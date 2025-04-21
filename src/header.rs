use itertools::Itertools;
use strum::{IntoEnumIterator, VariantMetadata};
use yew::{classes, function_component, html, Callback, Html, Properties};

use crate::page::Page;

#[derive(Properties, PartialEq)]
pub struct Props {
  pub page: Page,
  pub change_page: Callback<Page>,
}

#[function_component(Header)]
pub fn header(props: &Props) -> Html {
  let pages = Page::iter()
    .map(|page| {
      let change_page = props.change_page.clone();
      html! {
        <div
          class={classes!("menuitem")}
          onclick={move |_| change_page.emit(page)}
        >{page.variant_name()}</div>
      }
    })
    .collect_vec();

  html! {
    <div class={classes!("header")}>
      <div class={classes!("title")}>{ "cknittel.com" }</div>
      <div class={classes!("menu")}>
        {pages}
      </div>
    </div>
  }
}

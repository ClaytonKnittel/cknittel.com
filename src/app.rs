use yew::{classes, function_component, html, use_state_eq, Callback, Html, Properties};

#[derive(Clone, PartialEq, Eq)]
enum Page {
  Home,
  Games,
  Projects,
}

#[derive(Properties, PartialEq)]
struct Props {
  page: Page,
  change_page: Callback<Page>,
}

#[function_component(Header)]
fn header(props: &Props) -> Html {
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

#[function_component(Body)]
fn body() -> Html {
  html! {
    <div class={classes!("body")}>{ "contents" }</div>
  }
}

#[function_component(App)]
pub fn app() -> Html {
  let page = use_state_eq(|| Page::Home);

  html! {
    <>
      <Header page={(*page).clone()} change_page={move |page_id| page.set(page_id)} />
      <Body />
    </>
  }
}

use yew::{classes, function_component, html, Html};

#[function_component(HomePage)]
pub fn home_page() -> Html {
  html! {
    <div class={classes!("home")}>
      { "Welcome friend. Here to play games or learn more about me?" }
    </div>
  }
}

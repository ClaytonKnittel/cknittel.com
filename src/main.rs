mod app;
mod body;
mod header;
mod page;
mod pages;
mod web_util;

use app::App;

fn main() {
  yew::Renderer::<App>::new().render();
}

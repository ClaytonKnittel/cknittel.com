mod app;
mod body;
mod header;
mod page;
mod pages;

use app::App;

fn main() {
  yew::Renderer::<App>::new().render();
}

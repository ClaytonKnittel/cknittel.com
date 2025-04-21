mod app;
mod body;
mod header;
mod page;

use app::App;

fn main() {
  yew::Renderer::<App>::new().render();
}

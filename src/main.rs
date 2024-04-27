mod app;
mod header; // Include the header module

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}

use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
      <>
        <h1 class="brand">{"Crate Crafts"}<span class="material-symbols-outlined">{"local_bar"}</span></h1>
        <div class="pill-nav">
          <a class="active" href="#home">{"Home"}</a>
          <a href="#news">{"News"}</a>
          <a href="#contact">{"Contact"}</a>
          <a href="#about">{"About"}</a>
          <a href="#cart"><span class="material-symbols-outlined">{"shopping_cart"}</span></a>
        </div>
      </>
    }
}

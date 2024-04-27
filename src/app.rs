use yew::prelude::*;
use crate::header::Header; // Import the Header component

struct Product {
    name: String,
    image: String,
    price: f32,
}

#[function_component(App)]
pub fn app() -> Html {
    let products: Vec<Product> = vec![
        Product { name: "Cabernet Sauvignon".to_string(), image: "https://www.havenwines.co.ke/wp-content/uploads/2021/08/8008900001037.jpg".to_string(), price: 20.0 },
        Product { name: "Merlot".to_string(), image: "https://assets.houseofwine.gr/media/catalog/product/cache/04a61031f3eed06cbf15d459d0b61ca5/m/e/merlotboutaris.jpg".to_string(), price: 25.0 },
        // ... add the rest of your products here
        Product { name: "Cabernet Sauvignon".to_string(), image: "https://www.havenwines.co.ke/wp-content/uploads/2021/08/8008900001037.jpg".to_string(), price: 20.0 },
        Product { name: "Merlot".to_string(), image: "https://assets.houseofwine.gr/media/catalog/product/cache/04a61031f3eed06cbf15d459d0b61ca5/m/e/merlotboutaris.jpg".to_string(), price: 25.0 },
        // ... add the rest of your products here
        Product { name: "Cabernet Sauvignon".to_string(), image: "https://www.havenwines.co.ke/wp-content/uploads/2021/08/8008900001037.jpg".to_string(), price: 20.0 },
        Product { name: "Merlot".to_string(), image: "https://assets.houseofwine.gr/media/catalog/product/cache/04a61031f3eed06cbf15d459d0b61ca5/m/e/merlotboutaris.jpg".to_string(), price: 25.0 },
        // ... add the rest of your products here
    ];

    html! {
        <main>
        <Header /> // Use the Header component
            <div class="grid">
                { for products.iter().map(|product| html! { 
                    <div class="grid-item">
                        <img src={product.image.clone()} alt={product.name.clone()} />
                        <h4>{ product.name.clone() }</h4>
                        <p>{ format!("${:.2}", product.price) }</p>
                    </div> 
                }) }
            </div>
        </main>
    }
}

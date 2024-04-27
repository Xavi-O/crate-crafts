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
        Product {
            name: "Chardonnay".to_string(),
            image: "https://data.negoziodelvino.it/imgprodotto/chardonnay-doc_14505_zoom.jpg".to_string(),
            price: 19.99,
        },
        Product {
            name: "Riesling".to_string(),
            image: "https://storage.googleapis.com/drinksvine/products/nederburg-riesling.webp".to_string(),
            price: 24.99,
        },
        Product {
            name: "Pinot Noir".to_string(),
            image: "https://soys.co.ke/PImages/XBPBX-0.jpg".to_string(),
            price: 29.99,
        },
        Product {
            name: "Chenin Blanc".to_string(),
            image: "https://storage.googleapis.com/drinksvine/products/kwv-chenin-blanc.webp".to_string(),
            price: 17.99,
        },
        Product {
            name: "Albariño".to_string(),
            image: "https://www.ninetypluscellars.com/cdn/shop/files/Lot174_Albarino_ProductShot_334x800_8522b4f9-8559-4135-af6e-5e619c8cdcb2_334x.png?v=1698155641".to_string(),
            price: 22.99,
        },
        Product {
            name: "Aligoté".to_string(),
            image: "https://www.lamblin.com/wp-content/uploads/2020/09/bg-aligote.png".to_string(),
            price: 18.99,
        },
        Product {
            name: "Amarone".to_string(),
            image: "https://cdn.jarrolds.co.uk/products-temp/tommasi-amarone-della-valpolicella-classico-75cl%7Bw=1000,h=1000%7D.jpg".to_string(),
            price: 39.99,
        },
        Product {
            name: "Arneis".to_string(),
            image: "https://www.terredavino.it/wp-content/uploads/2016/07/Roero-Arneis-507x1024-1.jpg".to_string(),
            price: 21.99,
        },
        Product {
            name: "Asti Spumante".to_string(),
            image: "https://digitalcontent.api.tesco.com/v2/media/ghs/ea7beb6d-bc5b-49ce-941b-c09a914b3625/51b12ad0-0521-4ea1-bf67-a1db108af52d_1512016565.jpeg?h=960&w=960".to_string(),
            price: 16.99,
        },
        Product {
            name: "Auslese".to_string(),
            image: "https://applejack.com/site/images/Schmitt-Sohne-Riesling-Auslese-750-ml_1.png".to_string(),
            price: 29.99,
        },
        Product {
            name: "Banylus".to_string(),
            image: "https://assets.bonappetit.com/photos/5a0f389d64f83007e03339d1/1:1/w_2002,h_2002,c_limit/banyuls-dessert-wine-lede.jpg".to_string(),
            price: 24.99,
        },
        Product {
            name: "Barbaresco".to_string(),
            image: "https://www.triphammerwines.com/cdn/shop/products/66552900004_798x965.jpg?v=1688764759".to_string(),
            price: 39.99,
        },
        Product {
            name: "Bardolino".to_string(),
            image: "https://data.negoziodelvino.it/imgprodotto/bardolino-doc_8105.jpg".to_string(),
            price: 19.99,
        },
        Product {
            name: "Ace of Spades (Armand de Brignac)".to_string(),
            image: "https://www.wineswholesales.com.sg/cdn/shop/products/Armand_de_Brignac_Brut_Green_Champagne_grande.png?v=1569392094".to_string(),
            price: 299.99,
        },
        Product {
            name: "Angélus".to_string(),
            image: "https://www.bordeaux-tradition.com/wp-content/uploads/2018/05/angelus-sm.png".to_string(),
            price: 249.99,
        },
        Product {
            name: "Astralis Shiraz".to_string(),
            image: "https://clarendonhills.com.au/wp-content/uploads/2022/08/Clarendon-Hills-Astralis-Syrah-324x324.png".to_string(),
            price: 39.99,
        },
        Product {
            name: "Ausone".to_string(),
            image: "https://www.bordeaux-tradition.com/wp-content/uploads/2018/05/ausone-sm-ma.png".to_string(),
            price: 799.99,
        },
        Product {
            name: "Beaucastel".to_string(),
            image: "https://aitkenwines.com/cdn/shop/files/FA22EBCA-D73E-4475-9943-F0793AACCA47.jpg?v=1705513772".to_string(),
            price: 89.99,
        },
    ];

    html! {
        <main>
        <Header /> // Use the Header component
        <div class="grid">
            { for products.iter().map(|product| html! { 
                <div class="grid-item">
                    <div class="product-image">
                        <img src={product.image.clone()} alt={product.name.clone()} />
                    </div>
                    <div class="product-details">
                        <i class="product-name">{ product.name.clone() }</i>
                        <p class="product-price">{ format!("${:.2}", product.price) }</p>
                    </div>
                </div> 
            }) }
        </div>    
        </main>
    }
}

//! Product
//! ============
//!
//! Reference: <https://stripe.com/docs/api/subscriptions>
//!

use actix_web::{web, Error, HttpResponse};
use serde::Deserialize;
use stripe::{Client, CreateProduct, Product};
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct CreateProductParams {
    #[validate(length(min = 1))]
    name: String,
    metadata: Option<std::collections::HashMap<String, String>>,
}

pub async fn create_product(
    client: web::Data<Client>,
    params: web::Json<CreateProductParams>,
) -> Result<HttpResponse, Error> {
    let product = {
        let mut create_product = CreateProduct::new(&params.name);
        create_product.metadata = params.metadata.clone();
        Product::create(&client, create_product).await.unwrap()
    };
    Ok(HttpResponse::Ok().json(product))
}

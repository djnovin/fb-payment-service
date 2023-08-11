//! Subscription
//! ============
//!
//! Reference: <https://stripe.com/docs/api/subscriptions>
//!

use actix_web::{web, Error, HttpResponse};
use serde::{Deserialize, Serialize};
use stripe::{Client, CreateCustomer, Customer, PaymentMethodId};
use validator::Validate;

use crate::validations::validate_mobile::validate_australian_mobile_number;

#[derive(Serialize, Deserialize, Validate)]
pub struct CreateCustomerParams {
    address: Option<stripe::Address>,
    #[validate(email)]
    email: Option<String>,
    name: Option<String>,
    #[validate(custom = "validate_australian_mobile_number")]
    phone: Option<String>,
    payment_method: Option<PaymentMethodId>,
}

pub async fn create_customer(
    client: web::Data<Client>,
    params: web::Json<CreateCustomerParams>,
) -> Result<HttpResponse, Error> {
    let create_customer = CreateCustomer {
        address: params.address.clone(),
        email: params.email.as_deref(),
        name: params.name.as_deref(),
        phone: params.phone.as_deref(),
        payment_method: params.payment_method.clone(),
        ..Default::default()
    };

    match Customer::create(&client, create_customer).await {
        Ok(customer) => Ok(HttpResponse::Ok().json(customer)),
        Err(e) => {
            eprintln!("Failed to create customer: {:?}", e); // Log the error
            Ok(HttpResponse::InternalServerError().into())
        }
    }
}

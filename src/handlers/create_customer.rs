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
    let mut create_customer = CreateCustomer::default();

    create_customer.address = params.address.clone();

    if let Some(email) = &params.email {
        create_customer.email = Some(&**email);
    }

    if let Some(name) = &params.name {
        create_customer.name = Some(&**name);
    }

    if let Some(phone) = &params.phone {
        create_customer.phone = Some(&**phone);
    }

    create_customer.payment_method = params.payment_method.clone();

    match Customer::create(&client, create_customer).await {
        Ok(customer) => Ok(HttpResponse::Ok().json(customer)),
        Err(e) => {
            eprintln!("Failed to create customer: {:?}", e); // Log the error
            Ok(HttpResponse::InternalServerError().into())
        }
    }
}

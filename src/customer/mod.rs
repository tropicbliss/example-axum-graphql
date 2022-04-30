mod model;

use async_graphql::{EmptySubscription, Schema, SimpleObject};
pub use model::{Mutation, QueryRoot};
use slab::Slab;
use std::collections::HashMap;
use tokio::sync::RwLock;
use uuid::Uuid;

pub type CustomersSchema = Schema<QueryRoot, Mutation, EmptySubscription>;

#[derive(SimpleObject, Clone)]
pub struct Customer {
    id: String,
    name: String,
    email: String,
    age: u8,
}

pub struct Customers {
    customers: RwLock<Slab<Customer>>,
    customer_data: RwLock<HashMap<String, usize>>,
}

impl Customers {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let mut customers = Slab::new();
        let john = customers.insert(Customer {
            id: "1".to_string(),
            name: "John Doe".to_string(),
            email: "jdoe@gmail.com".to_string(),
            age: 35,
        });
        let steve = customers.insert(Customer {
            id: "2".to_string(),
            name: "Steve Smith".to_string(),
            email: "steve@gmail.com".to_string(),
            age: 25,
        });
        let sara = customers.insert(Customer {
            id: "3".to_string(),
            name: "Sara Williams".to_string(),
            email: "sara@gmail.com".to_string(),
            age: 32,
        });
        let mut customer_data = HashMap::new();
        customer_data.insert("1".to_string(), john);
        customer_data.insert("2".to_string(), steve);
        customer_data.insert("3".to_string(), sara);
        Self {
            customers: RwLock::new(customers),
            customer_data: RwLock::new(customer_data),
        }
    }

    pub async fn customer(&self, id: &str) -> Option<Customer> {
        let pk = {
            let customer_data = self.customer_data.read().await;
            *customer_data.get(id)?
        };
        let customers = self.customers.read().await;
        customers.get(pk).cloned()
    }

    pub async fn customers(&self) -> Vec<Customer> {
        let customers = self.customers.read().await;
        customers.iter().map(|(_, c)| c).cloned().collect()
    }

    pub async fn add_customer(&self, name: String, email: String, age: u8) -> Customer {
        let new_id = Uuid::new_v4().to_string();
        let new_customer = Customer {
            id: new_id.clone(),
            name,
            email,
            age,
        };
        let pk = {
            let mut customers = self.customers.write().await;
            customers.insert(new_customer.clone())
        };
        let mut customer_data = self.customer_data.write().await;
        customer_data.insert(new_id, pk);
        new_customer
    }

    pub async fn delete_customer(&self, id: String) -> Option<Customer> {
        let pk = {
            let mut customer_data = self.customer_data.write().await;
            customer_data.remove(&id)?
        };
        let mut customers = self.customers.write().await;
        Some(customers.remove(pk))
    }

    pub async fn edit_customer(
        &self,
        id: &str,
        name: Option<String>,
        email: Option<String>,
        age: Option<u8>,
    ) -> Option<Customer> {
        let pk = {
            let customer_data = self.customer_data.read().await;
            *customer_data.get(id)?
        };
        let mut customers = self.customers.write().await;
        let customer = customers.get_mut(pk)?;
        if let Some(name) = name {
            customer.name = name;
        }
        if let Some(email) = email {
            customer.email = email;
        }
        if let Some(age) = age {
            customer.age = age;
        }
        Some(customer.clone())
    }
}

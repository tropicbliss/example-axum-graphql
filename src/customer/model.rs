use super::{Customer, Customers};
use async_graphql::{Context, Object};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn customer(&self, ctx: &Context<'_>, id: String) -> Option<Customer> {
        ctx.data_unchecked::<Customers>().customer(&id).await
    }

    async fn customers(&self, ctx: &Context<'_>) -> Vec<Customer> {
        ctx.data_unchecked::<Customers>().customers().await
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn add_customer(
        &self,
        ctx: &Context<'_>,
        name: String,
        email: String,
        age: u8,
    ) -> Customer {
        ctx.data_unchecked::<Customers>()
            .add_customer(name, email, age)
            .await
    }

    async fn delete_customer(&self, ctx: &Context<'_>, id: String) -> Option<Customer> {
        ctx.data_unchecked::<Customers>().delete_customer(id).await
    }

    async fn edit_customer(
        &self,
        ctx: &Context<'_>,
        id: String,
        name: Option<String>,
        email: Option<String>,
        age: Option<u8>,
    ) -> Option<Customer> {
        ctx.data_unchecked::<Customers>()
            .edit_customer(&id, name, email, age)
            .await
    }
}

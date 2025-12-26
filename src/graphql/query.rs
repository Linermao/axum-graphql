use async_graphql::{ID, Object};

use crate::domain::user;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn hello_user(&self, id: ID, name: String) -> String {
        user::hello(id.as_str(), &name)
    }
}

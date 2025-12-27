use async_graphql::{ID, Object};

use crate::domain::user;

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    // Identical to QueryRoot at the Rust level.
    // The Query/Mutation split is a GraphQL semantic distinction
    // (execution order and side-effect expectations), not a syntax one. 
    async fn hello_user(&self, id: ID, name: String) -> String {
        user::hello(id.as_str(), &name)
    }
}

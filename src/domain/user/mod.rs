pub mod graphql;

pub fn hello(id: &str, name: &str) -> String {
    format!("hello! user: {}, ID: {}", name, id)
}

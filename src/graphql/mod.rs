use async_graphql::{
    Schema,
    extensions::{ApolloTracing, Tracing},
};
use sqlx::PgPool;
use tokio::sync::broadcast;

use crate::{
    domain::project::{rf_canva::RfEvent, tree::TreeEvent},
    graphql::{mutation::MutationRoot, query::QueryRoot, subscription::SubscriptionRoot},
};

pub mod mutation;
pub mod query;
pub mod subscription;

pub type AppSchema = Schema<QueryRoot, MutationRoot, SubscriptionRoot>;

pub struct AppState {
    pub db: PgPool,
    pub events: EventBus,
}

pub struct EventBus {
    tree_event: broadcast::Sender<TreeEvent>,
    rf_event: broadcast::Sender<RfEvent>,
}

impl EventBus {
    pub fn new(buffer: usize) -> Self {
        let (tree_event, _) = broadcast::channel(buffer);
        let (rf_event, _) = broadcast::channel(buffer);
        Self {
            tree_event,
            rf_event,
        }
    }

    /* -------- Tree -------- */

    pub fn publish_tree(&self, event: TreeEvent) {
        let _ = self.tree_event.send(event);
    }

    pub fn subscribe_tree(&self) -> broadcast::Receiver<TreeEvent> {
        self.tree_event.subscribe()
    }

    pub fn publish_rf(&self, event: RfEvent) {
        let _ = self.rf_event.send(event);
    }

    pub fn subscribe_rf(&self) -> broadcast::Receiver<RfEvent> {
        self.rf_event.subscribe()
    }
}

pub fn build_schema(pool: PgPool) -> AppSchema {
    let events = EventBus::new(1024);

    let state = AppState { db: pool, events };

    Schema::build(QueryRoot, MutationRoot, SubscriptionRoot)
        .extension(Tracing)
        .extension(ApolloTracing)
        .data(state)
        .finish()
}

use async_graphql::*;
use futures_util::Stream;
use std::time::Duration;
use tokio_stream::StreamExt;
use uuid::Uuid;

use crate::{
    domain::project::{rf_canva::RfEvent, tree::TreeEvent},
    graphql::AppState,
};

pub struct SubscriptionRoot;

#[Subscription]
impl SubscriptionRoot {
    /// test subscription
    async fn integers(&self, #[graphql(default = 1)] step: i32) -> impl Stream<Item = i32> {
        let mut value = 0;
        tokio_stream::wrappers::IntervalStream::new(tokio::time::interval(Duration::from_secs(1)))
            .map(move |_| {
                value += step;
                value
            })
    }

    async fn tree_events(
        &self,
        ctx: &Context<'_>,
        project_id: Uuid,
    ) -> impl Stream<Item = TreeEvent> {
        let state = ctx.data::<AppState>().unwrap();
        let mut rx = state.events.subscribe_tree();

        async_stream::stream! {
            while let Ok(ev) = rx.recv().await {
                if ev.project_id() == project_id {
                    yield ev;
                }
            }
        }
    }

    async fn rf_events(&self, ctx: &Context<'_>, canva_id: Uuid) -> impl Stream<Item = RfEvent> {
        let state = ctx.data::<AppState>().unwrap();
        let mut rx = state.events.subscribe_rf();

        async_stream::stream! {
            while let Ok(ev) = rx.recv().await {
                if ev.canva_id() == canva_id {
                    yield ev;
                }
            }
        }
    }
}

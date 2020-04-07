use juniper::{RootNode, EmptyMutation};
use std::sync::Arc;

pub struct Query;

graphql_object!(Query: () |&self| {
    field apiVersion() -> &str {
        "1.0"
    }
});

pub fn root_node<'a>() -> Arc<RootNode<'a, Query, EmptyMutation<()>>> {
    Arc::new(RootNode::new(Query, EmptyMutation::<()>::new()))
}
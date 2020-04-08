use juniper::{RootNode, EmptyMutation};

pub struct Query;

graphql_object!(Query: () |&self| {
    field apiVersion() -> &str {
        "1.0"
    }
});

pub fn root_node() -> RootNode<'static, Query, EmptyMutation<()>> {
    RootNode::new(Query, EmptyMutation::<()>::new())
}
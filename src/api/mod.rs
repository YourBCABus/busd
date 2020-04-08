use juniper::{RootNode, EmptyMutation};

pub struct Query;

graphql_object!(Query: () |&self| {
    field apiVersion() -> &str {
        "1.0"
    }
});

pub type Schema = RootNode<'static, Query, EmptyMutation<()>>;

pub fn schema() -> Schema {
    RootNode::new(Query, EmptyMutation::<()>::new())
}
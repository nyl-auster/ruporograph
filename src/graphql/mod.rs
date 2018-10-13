pub mod schema;
pub mod user;

// A root schema consists of a query and a mutation.
// Request queries can be executed against a RootNode.
pub type Schema = juniper::RootNode<'static, schema::Query, schema::Mutation>;

pub fn build_schema() -> Schema {
  Schema::new(schema::Query, schema::Mutation)
}

use business::user::{User};
use graphql::user::{UserInput, UserCreate, field_user, field_user_create};
use app::Ctx;
use juniper::{ FieldError };

// Queries
pub struct Query;

graphql_object!(Query: Ctx as "Query" |&self| {
    description: "The root query object of the schema"

    field Hello() -> &str {
      "Just saying hello."
    }

    field User(&executor, id: i32) -> Result<User, FieldError> {
      field_user(&executor, id)
    }
     
});

// Mutations
pub struct Mutation;

graphql_object!(Mutation: Ctx |&self| {
    field Hello() -> &str {
      "Just saying hello from mutation"
    }

    field UserCreate(&executor, userInput: UserInput) -> Result<UserCreate, FieldError> {
      field_user_create(&executor, userInput)
    }

});


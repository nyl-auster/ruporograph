use app::Ctx;
use business::user::{get_user_by_id, insert_user, User};
use juniper::{FieldError, Value};

// define custom resolvers for each field of User
graphql_object!(User: () as "Utilisateur" |&self| {
    field name() -> &str {
        self.name.as_str()
    }
    field mail() -> &str {
        self.mail.as_str()
    }
    field id() -> Option<i32> {
      self.id
    }
});

#[derive(Debug, GraphQLInputObject)]
pub struct UserInput {
  pub name: String,
  pub mail: String,
}

// Resolver for User Query
pub fn field_user(executor: &&juniper::Executor<'_, Ctx>, id: i32) -> Result<User, FieldError> {
  let user = get_user_by_id(&executor.context(), id);
  match user {
    Some(v) => Ok(v),
    None => Err(FieldError::new("Utitisateur non trouvé", Value::null())),
  }
}

#[derive(Debug, GraphQLObject)]
pub struct UserCreate {
  rows_updated: i32,
}

// Resolver for UserCreate mutation
pub fn field_user_create(
  executor: &&juniper::Executor<'_, Ctx>,
  user_input: UserInput,
) -> Result<UserCreate, FieldError> {
  let user = User {
    id: None,
    name: user_input.name,
    mail: user_input.mail,
  };
  match insert_user(&executor.context(), &user) {
    Ok(rows_updated) => Ok(UserCreate {
      rows_updated: rows_updated as i32,
    }),
    Err(message) => Err(FieldError::new(message, Value::null())),
  }
}

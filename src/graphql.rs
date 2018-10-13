use helpers::Ctx;
use user::{get_user_by_id, insert_user, User};
use juniper::{FieldError, Value};

/*******************
 * Field Query
 *******************/
pub struct Query;

graphql_object!(Query: Ctx as "Query" |&self| {
    description: "L'objet query à la racine du schema"

    field Hello() -> &str {
      "Just saying hello !"
    }

    field User(&executor, id: i32) -> Result<User, FieldError> {
      let user = get_user_by_id(&executor.context(), id);
      match user {
        Some(v) => Ok(v),
        None => Err(FieldError::new("Utitisateur non trouvé", Value::null())),
      }
    }
     
});

/*******************
 * Field Mutations
 *******************/
pub struct Mutation;

graphql_object!(Mutation: Ctx |&self| {
    description: "L'objet mutation à la racine du schema"

    field UserCreate(&executor, user_input: UserInput) -> Result<UserCreate, FieldError> {
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

});

/*******************
 * Field User
 *******************/

// déclarer des resolvers custom pour chaque champ
// de notre structure "user::User"
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

/*************************
 * Réponse de UserCreate
 **************************/
#[derive(Debug, GraphQLObject)]
pub struct UserCreate {
  rows_updated: i32,
}

/*************************
 * Argument pour UserCreate
 **************************/
#[derive(Debug, GraphQLInputObject)]
pub struct UserInput {
  pub name: String,
  pub mail: String,
}

/*************************
 * Création du schema
 **************************/
pub type Schema = juniper::RootNode<'static, Query, Mutation>;

pub fn build_schema() -> Schema {
  Schema::new(Query, Mutation)
}

use helpers::Ctx;

#[derive(Debug)]
pub struct User {
  pub id: Option<i32>,
  pub name: String,
  pub mail: String,
}

pub fn get_user_by_id(ctx: &Ctx, id: i32) -> Option<User> {
  let mut result = None;
  for row in &ctx
    .db_connect()
    .query("SELECT id, name, mail FROM users WHERE id = $1", &[&id])
    .expect(&format!(
      "erreur en cherchant l'utilisateur avec l'id {:#?}",
      &id
    )) {
    result = Some(User {
      id: Some(row.get("id")),
      name: row.get("name"),
      mail: row.get("mail"),
    });
  }
  result
}

pub fn insert_user(ctx: &Ctx, user: &User) -> Result<u64, String> {
  let rows_affected = ctx
    .db_connect()
    .execute(
      "INSERT INTO users (name, mail) VALUES ($1, $2)",
      &[&user.name, &user.mail],
    )
    .expect(&format!("erreur en insérant l'utilisateur {:#?}", &user));
  if rows_affected > 0 {
    Ok(rows_affected)
  } else {
    Err(String::from(
      "Aucune rangée modifiée en base, l'insert semble avoir échoué",
    ))
  }
}

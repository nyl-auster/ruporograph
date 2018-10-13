use app::Ctx;

pub fn run(ctx: &Ctx) {
  println!("création de la table utilisateur");
  let connect = ctx.db_pool.get().unwrap();
  connect
    .execute(
      "CREATE TABLE users (
    id  SERIAL PRIMARY KEY,
    name  VARCHAR NOT NULL,
    mail  VARCHAR NOT NULL
    )",
      &[],
    )
    .expect("erreur en créant la table users");

  println!("création de la table test");
  let connect = ctx.db_pool.get().unwrap();
  connect
    .execute(
      "CREATE TABLE test (
    id  SERIAL PRIMARY KEY,
    name  VARCHAR NOT NULL
    )",
      &[],
    )
    .expect("erreur en créant la table test");
}

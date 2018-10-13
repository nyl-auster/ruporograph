use app::Ctx;

pub fn run(ctx: &Ctx) {
  let connect = ctx.db_pool.get().unwrap();
  println!("suppression de la table users");
  connect
    .execute("DROP TABLE IF EXISTS users", &[])
    .expect("erreur en supprimant la table users");

  println!("suppression de la table test");
  connect
    .execute("DROP TABLE IF EXISTS test", &[])
    .expect("erreur en supprimant la table test");
}

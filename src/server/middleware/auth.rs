use std::rc::Rc;

use crate::db::PgPool;

pub struct AuthService {
    db_pool: Rc<PgPool>,
}

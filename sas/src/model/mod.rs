pub mod invitation;
pub mod user;

use diesel::{r2d2::ConnectionManager, PgConnection};

// 类型别名 Pool
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub use invitation::Invitation;
pub use user::{SlimUser, User};

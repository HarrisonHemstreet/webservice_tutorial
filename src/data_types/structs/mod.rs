use serde::{Deserialize, Serialize};

pub mod blog;
pub use self::blog::Blog;

pub mod error_message;
pub use self::error_message::ErrorMessage;

pub mod auth;

mod tag;
pub use self::tag::Tag;
pub use self::tag::AssocTable;
pub use self::tag::TagQueryParams;

pub use self::auth::Auth;
pub use self::auth::Status;

#[derive(Serialize, Deserialize, Debug)]
pub struct Id {
    pub id: Option<i32>,
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(id) = self.id {
            return write!(f, "Id: {}", id);
        }
        write!(f, "Id: {}", "None")
    }
}

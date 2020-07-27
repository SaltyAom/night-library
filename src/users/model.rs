use serde::{Deserialize, Serialize};

use diesel::prelude::*;

use crate::schema::users;
use crate::schema::users::dsl::*;

use juniper::GraphQLObject;

// In-case for change in the future
#[derive(Clone, Deserialize, Serialize, Queryable, Insertable, GraphQLObject)]
pub struct User {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UserForm {
    pub username: String,
    pub password: String,
}

impl User {
    pub fn register(&self, connection: &PgConnection) -> Result<User, diesel::result::Error> {
        let new_user = User {
            username: self.username.to_owned(),
            password: self.password.to_owned(),
        };

        diesel::insert_into(users)
            .values(&new_user)
            .execute(connection)?;

        Ok(new_user)
    }

    pub fn login(&self, connection: &PgConnection) -> Result<Option<User>, diesel::result::Error> {
        let requested_user = users
            .filter(username.eq(self.username.to_owned()))
            .filter(password.eq(self.password.to_owned()))
            .first::<User>(connection)
            .optional()?;

        Ok(requested_user)
    }
}

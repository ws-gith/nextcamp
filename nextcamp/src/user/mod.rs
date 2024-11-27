mod password;

use crate::{Hasher, Sensitive};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum Error {
    FailedToCreateUser,
}
impl_error_display!(Error);

type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(new, Debug, Deserialize, Serialize)]
pub struct Data<T> {
    pub id: T,
    pub name: String,
    pub email: String,
    pub password: Sensitive<String>,
}

#[async_trait::async_trait]
pub trait Repository {
    type Error;
    type ID;

    /// Create user
    ///
    /// # Notes
    /// * takes in `UserDto` which the id will always be default of T
    async fn create_user(&self, name: &str, email: &str, password: &str) -> Result<Data<Self::ID>, Self::Error>;
    async fn get_user_by_name(&self, name: String) -> Result<Data<Self::ID>, Self::Error>;
    async fn get_user_by_id(&self, id: Self::ID) -> Result<Data<Self::ID>, Self::Error>;

    // update user details
    async fn update_user(&self, user: &Data<Self::ID>) -> Result<Data<Self::ID>, Self::Error>;

    //  delete user by id
    async fn delete_user_by_id(&self, id: Self::ID) -> Result<Data<Self::ID>, Self::Error>;
}

// #[async_trait::async_trait]
// pub trait Validator<ID> {
//     type Error;

//     async fn validate(&self, user: Data<ID>) -> Result<(), Self::Error>;
// }

// Contain what user in our system
#[cfg_attr(debug_assertions, allow(dead_code))]
#[derive(new)]
pub struct User<U: Repository, H: Hasher> {
    password_hasher: H,
    repository: U,
    // user_validator: Option<Vec<Box<dyn Validator<U::ID>>>>,
}

impl<R: Repository, H: Hasher> User<R, H> {
    pub async fn create_user(&self, name: &str, email: &str, password: &str) -> Result<Data<R::ID>> {
        let password = self.password_hasher.hash(password);

        let db_user = self
            .repository
            .create_user(name, email, &password)
            .await
            .map_err(|_| Error::FailedToCreateUser)?;

        Ok(db_user)
    }
}

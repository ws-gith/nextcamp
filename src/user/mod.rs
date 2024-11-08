use serde::{Deserialize, Serialize};

#[derive(new, Debug, Deserialize, Serialize)]
pub struct UserDto<T> {
    id: T,
    name: String,
    email: String,
}

#[async_trait::async_trait]
pub trait UserRepository<T> {
    type Error;

    async fn get_user_by_email(&self, email: String) -> Result<UserDto<T>, Self::Error>;

    async fn get_user_by_name(&self, name: String) -> Result<UserDto<T>, Self::Error>;

    async fn get_user_by_id(&self, id: T) -> Result<UserDto<T>, Self::Error>;
}

// Contain what user in our system
#[derive(new)]
pub struct User<Repository: UserRepository<String>> {
    /**
     *
     * Role
     * Password Manager
     */
    repository: Repository,
}

impl<Repository: UserRepository<String>> User<Repository> {
    /***
     * Create User
     * Update User - id - name - email
     * Delete User - id - name - email
     * Read User - id - name - email
     *
     */
}

use axum::{
    http::HeaderMap,
    response::{IntoResponse, Response},
};

#[derive(Debug)]
pub(super) enum Error {}
impl_error_display!(Error);

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        todo!()
    }
}

type Result<T> = core::result::Result<T, Error>;

pub(in crate::routes) mod sign_in {
    use super::Result;
    use axum::http::StatusCode;
    use axum_plus::{Body, OK};

    #[allow(dead_code)]
    mod post {
        #[derive(Debug, serde::Deserialize, validator::Validate)]
        pub struct Payload {
            name: String,

            #[validate(length(min = 10, message = "your password is too short"))]
            password: String,
        }
    }

    #[cfg_attr(debug_assertions, axum::debug_handler)]
    pub(in crate::routes) async fn POST(Body(_payload): Body<post::Payload>) -> Result<StatusCode> {
        // check user details and handle session if failed
        Ok(OK)
    }
}

pub(in crate::routes) mod sign_up {
    use super::Result;
    use axum::http::StatusCode;
    use axum_plus::{Body, OK};

    #[allow(dead_code)]
    mod post {
        #[derive(Debug, serde::Deserialize, validator::Validate)]
        pub struct Payload {
            name: String,

            #[validate(email(message = "invalid email address"))]
            email: String,

            #[validate(length(min = 10, message = "your password is too short"))]
            password: String,
        }
    }

    #[cfg_attr(debug_assertions, axum::debug_handler)]
    pub(in crate::routes) async fn POST(Body(_payload): Body<post::Payload>) -> Result<StatusCode> {
        // call user manager to sign_up
        // create session
        Ok(OK)
    }
}

pub(in crate::routes) mod reset_password {
    use super::{HeaderMap, Result};

    #[allow(dead_code)]
    mod post {
        // Do all post shit here
        pub struct Payload {}
    }

    #[cfg_attr(debug_assertions, axum::debug_handler)]
    pub(in crate::routes) async fn POST(header: HeaderMap) -> Result<&'static str> {
        println!("{:#?}", header);

        Ok("Ok, this is auth!")
    }
}

use actix_web::{
    Error, HttpMessage,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
    error,
};
use jsonwebtoken::{DecodingKey, Validation, decode};
use std::future::{Ready, ready};

use futures_util::future::LocalBoxFuture;

use crate::{
    config::token::TokenVars,
    shared::etities::userdb::{User, UserClaims},
};

pub struct UserJwt;

impl<S, B> Transform<S, ServiceRequest> for UserJwt
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = UserJwtMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(UserJwtMiddleware { service }))
    }
}

pub struct UserJwtMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for UserJwtMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let secret = match TokenVars::from_env() {
            Ok(s) => s,
            Err(e) => {
                let err = error::ErrorInternalServerError(e).into();
                return Box::pin(async { Err(err) });
            }
        };

        let tk = match req.headers().get("Authorization") {
            Some(tk) => tk,
            None => {
                let err = error::ErrorUnauthorized("Authorization header not found").into();

                return Box::pin(async { Err(err) });
            }
        };

        let tk = match tk.to_str() {
            Ok(tk) => tk,
            Err(e) => {
                let err = error::ErrorInternalServerError(e.to_string()).into();

                return Box::pin(async { Err(err) });
            }
        };

        let mut validation = Validation::default();
        validation.validate_exp = false;

        let user = match decode::<UserClaims<User>>(
            &tk,
            &DecodingKey::from_secret(secret.seed().as_ref()),
            &validation,
        ) {
            Ok(u) => u.claims.user,
            Err(e) => {
                let err =
                    error::ErrorUnauthorized(format!("Invalid token: {}", e.to_string())).into();

                return Box::pin(async { Err(err) });
            }
        };

        req.extensions_mut().insert(user.id);

        // println!("tk: {user:#?}");

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;

            println!("Hi from response");
            Ok(res)
        })
    }
}

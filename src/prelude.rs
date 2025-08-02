pub use async_trait::async_trait;
pub use axum::{
    extract::{Form, Path, State},
    response::{IntoResponse, Response},
    routing::{delete, get, head, options, patch, post, put, trace},
};
pub use axum_extra::extract::cookie;
pub use chrono::NaiveDateTime as DateTime;
pub use include_dir::{include_dir, Dir};
// some types required for controller generators
#[cfg(feature = "with-db")]
pub use sea_orm::prelude::{Date, DateTimeUtc, DateTimeWithTimeZone, Decimal, Uuid};
#[cfg(feature = "with-db")]
pub use sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, ActiveValue, ColumnTrait, ConnectionTrait,
    DatabaseConnection, DbErr, EntityTrait, IntoActiveModel, ModelTrait, QueryFilter, Set,
    TransactionTrait,
};
// sugar for controller views to use `data!({"item": ..})` instead of `json!`
pub use serde_json::json as data;

#[cfg(feature = "auth_jwt")]
pub use crate::controller::extractor::auth;
pub use crate::controller::extractor::{
    shared_store::SharedStore,
    validate::{JsonValidate, JsonValidateWithMessage},
};
#[cfg(feature = "with-db")]
pub use crate::model::{query, Authenticable, ModelError, ModelResult};
pub use crate::{
    app::{AppContext, Initializer},
    bgworker::{BackgroundWorker, Queue},
    controller::{
        bad_request, format,
        middleware::{
            format::{Format, RespondTo},
            remote_ip::RemoteIP,
        },
        not_found, unauthorized,
        views::{engines::TeraView, ViewEngine, ViewRenderer},
        Json, Routes,
    },
    errors::Error,
    mailer,
    mailer::Mailer,
    task::{self, Task, TaskInfo},
    validation::{self, Validatable},
    validator::Validate,
    Result,
};
#[cfg(feature = "with-db")]
pub mod model {
    pub use crate::model::query;
}
#[cfg(feature = "testing")]
pub use crate::testing::prelude::*;

pub use macros::*;
mod macros {

    // The "magical" macro to generate the `From` implementation.
    // Place this at the top of your models file.
    #[macro_export]
    macro_rules! auto_map_to_active_model {
        ($from:ident, $to:ident, { $($field:ident),* }) => {
            impl From<$from> for $to {
                fn from(params: $from) -> Self {
                    Self {
                        $(
                            $field: sea_orm::ActiveValue::Set(params.$field),
                        )*
                        ..Default::default()
                    }
                }
            }
        };
    }
    /// Maps fields from a sea_orm::Model to a custom struct.
    ///
    /// # Arguments
    ///
    /// * `$from`: The source `model::Model` struct.
    /// * `$to`: The destination struct.
    /// * `{ $($field:ident),* }`: A comma-separated list of fields to map.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `user::Model` is a SeaORM model and `UserResponse` is your custom struct.
    /// auto_map_from_model!(user::Model, UserResponse, { id, username, email });
    ///
    /// // Now you can convert a model instance into your response struct:
    /// // let user_model: user::Model = ...;
    /// // let user_response: UserResponse = user_model.into();
    /// ```
    ///
    #[macro_export]
    macro_rules! auto_map_from_model {
        ($from:ident, $to:ident, { $($field:ident),* }) => {
            impl From<$from> for $to {
                fn from(model: $from) -> Self {
                    Self {
                        $(
                            $field: model.$field,
                        )*
                        ..Default::default()
                    }
                }
            }
        };
    }
}

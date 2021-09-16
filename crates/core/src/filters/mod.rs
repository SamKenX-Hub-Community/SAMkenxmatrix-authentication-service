// Copyright 2021 The Matrix.org Foundation C.I.C.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![allow(clippy::unused_async)] // Some warp filters need that

pub mod csrf;
// mod errors;
pub mod authenticate;
pub mod client;
pub mod cookies;
pub mod database;
pub mod headers;
pub mod session;

use std::convert::Infallible;

use warp::Filter;

pub use self::csrf::CsrfToken;
use crate::{
    config::{KeySet, OAuth2Config},
    templates::Templates,
};

#[must_use] pub fn with_templates(
    templates: &Templates,
) -> impl Filter<Extract = (Templates,), Error = Infallible> + Clone + Send + Sync + 'static {
    let templates = templates.clone();
    warp::any().map(move || templates.clone())
}

#[must_use] pub fn with_keys(
    oauth2_config: &OAuth2Config,
) -> impl Filter<Extract = (KeySet,), Error = Infallible> + Clone + Send + Sync + 'static {
    let keyset = oauth2_config.keys.clone();
    warp::any().map(move || keyset.clone())
}
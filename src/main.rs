#![allow(non_snake_case)]

pub mod app;
pub mod articles;
pub mod blog;
pub mod contact;
pub mod home;
pub mod projects;

use crate::app::App;
use leptos::{mount_to_body, view};

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        view! { <App/> }
    });
}

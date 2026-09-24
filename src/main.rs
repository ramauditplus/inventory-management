use leptos::mount::mount_to_body;

mod app;
mod components;
mod pages;
mod models;

fn main() {
    mount_to_body(app::App);
}
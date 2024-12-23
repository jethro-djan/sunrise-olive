use leptos::{mount_to_body, view};
use rs_oblivion_times::app;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <app::App/> })
}

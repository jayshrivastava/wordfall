use leptos::*;
use stylance::import_crate_style;

import_crate_style!(styles, "./src/styles.module.scss");

#[component]
pub fn Header() -> impl IntoView {
  view! {
    <p class=styles::header> wordfall  </p>
  }
}
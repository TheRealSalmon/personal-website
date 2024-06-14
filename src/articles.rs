#![allow(non_snake_case)]

use leptos::*;

#[component]
pub fn LivingWithCancer() -> impl IntoView {
    let lines = include_str!("../articles/living-with-cancer.txt").split("\n\n");
    view! {
      <br/>
      <br/>
      <br/>
      <br/>
      <p class="text-base italic">"Living with Cancer"</p>
      <p class="text-sm italic">"June 13th, 2024"</p>

      {lines.into_iter().map(|line| view! { <p class="text-xl pt-6">{line}</p> }).collect_view()}
    }
}

#![allow(non_snake_case)]

use leptos::*;

#[component]
fn Article(title: String, date: String, lines: Vec<String>) -> impl IntoView {
    view! {
      <br/>
      <br/>
      <br/>
      <br/>
      <p class="text-base italic">{&title}</p>
      <p class="text-sm italic">{&date}</p>

      {lines
          .iter()
          .map(|line| view! { <p class="text-base sm:text-xl pt-6">{line}</p> })
          .collect_view()}
    }
}

#[component]
pub fn WatchingMyselfRecover() -> impl IntoView {
    let lines: Vec<_> = include_str!("../articles/watching-myself-recover.txt")
        .split("\n\n")
        .map(|s| s.to_string())
        .collect();
    view! {
      <Article
        title="Watching Myself Recover".to_string()
        date="June 17th, 2024".to_string()
        lines=lines
      />
    }
}

#[component]
pub fn LivingWithCancer() -> impl IntoView {
    let lines: Vec<_> = include_str!("../articles/living-with-cancer.txt")
        .split("\n\n")
        .map(|s| s.to_string())
        .collect();
    view! {
      <Article
        title="Living with Cancer".to_string()
        date="June 13th, 2024".to_string()
        lines=lines
      />
    }
}

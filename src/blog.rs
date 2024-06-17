use crate::articles::*;
use leptos::*;
use leptos_router::*;

#[component(transparent)]
pub fn BlogRoutes() -> impl IntoView {
    view! {
      <Route path="/blog" view=|| view! { <Outlet/> }>
        <Route path="" view=Blog/>
        <Route path="living-with-cancer" view=LivingWithCancer/>
      </Route>
    }
}

#[component]
pub fn Blog() -> impl IntoView {
    view! {
      <br/>
      <br/>
      <br/>
      <br/>
      <p class="text-sm italic">"Articles"</p>
      <a class="text-xl pt-4 hover:text-slate-400" href="/blog/living-with-cancer">
        "June 13th 2024, Living with Cancer"
      </a>
    }
}

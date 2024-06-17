use crate::articles::*;
use leptos::*;
use leptos_router::*;

#[component(transparent)]
pub fn BlogRoutes() -> impl IntoView {
    view! {
      <Route path="/blog" view=|| view! { <Outlet/> }>
        <Route path="" view=Blog/>
        <Route path="living-with-cancer" view=LivingWithCancer/>
        <Route path="watching-myself-recover" view=WatchingMyselfRecover/>
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
      <a href="/blog/watching-myself-recover">
        <p class="text-xl pt-8 hover:text-slate-400">"June 17th 2024, Watching Myself Recover"</p>
      </a>
      <a href="/blog/living-with-cancer">
        <p class="text-xl pt-8 hover:text-slate-400">"June 13th 2024, Living with Cancer"</p>
      </a>
    }
}

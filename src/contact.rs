use leptos::*;

#[component]
pub fn Contact() -> impl IntoView {
    view! {
      <nav class="flex flex-col absolute bottom-0 right-0">
        <a
          class="hover:text-slate-400"
          href="https://www.linkedin.com/in/therealsam/"
          target="_blank"
        >
          "LinkedIn"
        </a>
        <a class="hover:text-slate-400" href="https://github.com/TheRealSalmon" target="_blank">
          "GitHub"
        </a>
      </nav>
    }
}

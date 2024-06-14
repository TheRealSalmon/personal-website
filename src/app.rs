use crate::articles::*;
use crate::blog::Blog;
use crate::contact::Contact;
use crate::home::Home;
use crate::projects::Projects;
use leptos::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
      <Router>
        <div class="font-mono p-6 h-screen bg-gray-300">
          <div class="p-6 h-full border-2 border-slate-400 border-solid bg-neutral-300">
            <p class="text-3xl">"Sam Mun"</p>
            <p>"Cheminformatics and"</p>
            <p>"Computational Chemistry"</p>
            <nav class="pt-6 flex flex-col absolute top-32 left-12">
              <a class="hover:text-slate-400" href="/">
                "Home"
              </a>
              <a class="hover:text-slate-400" href="/projects">
                "Projects"
              </a>
              <a class="hover:text-slate-400" href="/contact">
                "Contact"
              </a>
              <a class="hover:text-slate-400" href="/blog">
                "Blog"
              </a>
            </nav>
            <div class="no-scrollbar flex flex-col text-right text-slate-600 w-5/12 h-5/6 overflow-hidden overflow-y-auto overscroll-contain absolute top-16 right-12">
              <Routes>
                <Route path="/" view=Home/>
                <Route path="/projects" view=Projects/>
                <Route path="/contact" view=Contact/>
                <Route path="/blog" view=Blog/>
                <Route path="/blog/living-with-cancer" view=LivingWithCancer/>
              </Routes>
            </div>
          </div>
        </div>
      </Router>
    }
}

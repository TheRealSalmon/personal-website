use crate::articles::*;
use crate::home::Home;
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
            <div class="no-scrollbar flex flex-col text-right text-slate-600 w-1/2 h-5/6 overflow-hidden overflow-y-auto overscroll-contain absolute top-16 right-12">
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

#[component]
pub fn Projects() -> impl IntoView {
    view! {
      <div class="no-scrollbar flex flex-col text-right text-slate-600 w-3/6 h-5/6 overflow-hidden overflow-y-auto overscroll-contain absolute top-16 right-12">
        <br/>
        <br/>
        <br/>
        <br/>
        <p class="text-sm italic">"Professional Work"</p>

        <p class="text-xl pt-4">"Virtual Screening Workflows (vina, openmm, amber) on AWS"</p>
        <p class="text-sm">"Proprietary Work"</p>

        <p class="text-xl pt-4">"DEL Hit Analysis Workflows"</p>
        <p class="text-sm">"Proprietary Work"</p>

        <p class="text-xl pt-4">"Prodrugs of Lenacapavir"</p>
        <a
          class="text-sm hover:text-slate-400"
          href="https://patents.google.com/patent/WO2023102239A1"
          target="_blank"
        >
          "Link to Patent"
        </a>

        <p class="text-sm italic pt-16">"Open-Source Work"</p>

        <p class="text-xl pt-4">"Learn Cheminformatics Interactively"</p>
        <a
          class="text-sm hover:text-slate-400"
          href="https://interactive-cheminformatics.streamlit.app"
          target="_blank"
        >
          "Link to Streamlit App"
        </a>

        <p class="text-xl pt-4">"molrs, Cheminformatics Toolkit in Rust"</p>
        <a
          class="text-sm hover:text-slate-400"
          href="https://github.com/molrs/molrs-core"
          target="_blank"
        >
          "Link to GitHub Repo"
        </a>
        <a
          class="text-sm hover:text-slate-400"
          href="https://molrs-book.vercel.app/index.html"
          target="_blank"
        >
          "Link to Book"
        </a>

        <p class="text-sm italic pt-16">"Academic Work"</p>

        <p class="text-xl pt-4">"Cell Membrane Voltage Reporting Dyes"</p>
        <a
          class="text-sm hover:text-slate-400"
          href="http://xlink.rsc.org/?DOI=d0cb00152j"
          target="_blank"
        >
          "Link to Publication"
        </a>
      </div>
    }
}

#[component]
pub fn Contact() -> impl IntoView {
    view! {
      <nav class="flex flex-col text-right text-slate-600 w-36 absolute bottom-12 right-12">
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

#[component]
pub fn Blog() -> impl IntoView {
    view! {
      <div class="no-scrollbar text-right text-slate-600 w-3/6 h-5/6 overflow-hidden overflow-y-auto overscroll-contain absolute top-16 right-12">
        <br/>
        <br/>
        <br/>
        <br/>
        <p class="text-sm italic">"Articles"</p>
        <a class="text-xl pt-4 hover:text-slate-400" href="/blog/living-with-cancer">
          "June 13th 2024, Living with Cancer"
        </a>
      </div>
    }
}

#![allow(non_snake_case)]

use leptos::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
      <Router>
        <div class="font-mono p-6 h-screen bg-zinc-300">
          <div class="p-6 h-full border-2 border-slate-400 border-solid">
            <p class="text-6xl sm:text-3xl">"Sam Mun"</p>
            <p class="text-xl sm:text-base">"Cheminformatics and"</p>
            <p class="text-xl sm:text-base">"Computational Chemistry"</p>
            <nav class="pt-6 flex flex-col">
              <a class="text-xl sm:text-base hover:text-slate-600" href="/">
                "Home"
              </a>
              <a class="text-xl sm:text-base hover:text-slate-600" href="/projects">
                "Projects"
              </a>
              <a class="text-xl sm:text-base hover:text-slate-600" href="/contact">
                "Contact"
              </a>
            </nav>
            <Routes>
              <Route path="/" view=Home/>
              <Route path="/projects" view=Projects/>
              <Route path="/contact" view=Contact/>
            </Routes>
          </div>
        </div>
      </Router>
    }
}

#[component]
pub fn Home() -> impl IntoView {
    view! {
      <p class="text-xl sm:text-sm w-36 absolute bottom-12 right-12">
        "
        Jack-of-all-trades with broad experience across computational drug
        discovery. Deep expertise in custom virtual screening workflows and
        DELs.
        "
      </p>
    }
}

#[component]
pub fn Projects() -> impl IntoView {
    view! {
      <div class="no-scrollbar text-right w-3/6 h-5/6 overflow-hidden overflow-y-auto overscroll-contain absolute top-16 right-12">
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
          class="text-sm hover:text-slate-600"
          href="https://patents.google.com/patent/WO2023102239A1"
          target="_blank"
        >
          "Link to Patent"
        </a>

        <p class="text-sm italic pt-16">"Open-Source Work"</p>

        <p class="text-xl pt-4">"Learn Cheminformatics Interactively"</p>
        <a
          class="text-sm hover:text-slate-600"
          href="https://interactive-cheminformatics.streamlit.app"
          target="_blank"
        >
          "Link to Streamlit App"
        </a>

        <p class="text-xl pt-4">"molrs, Cheminformatics Toolkit in Rust"</p>
        <a
          class="text-sm hover:text-slate-600"
          href="https://github.com/molrs/molrs-core"
          target="_blank"
        >
          "Link to GitHub Repo"
        </a>

        <p class="text-sm italic pt-16">"Academic Work"</p>

        <p class="text-xl pt-4">"Cell Membrane Voltage Reporting Dyes"</p>
        <a
          class="text-sm hover:text-slate-600"
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
      <nav class="flex flex-col text-right w-36 absolute bottom-12 right-12">
        <a
          class="text-xl sm:text-sm hover:text-slate-600"
          href="https://www.linkedin.com/in/therealsam/"
          target="_blank"
        >
          "LinkedIn"
        </a>
        <a
          class="text-xl sm:text-sm hover:text-slate-600"
          href="https://github.com/TheRealSalmon"
          target="_blank"
        >
          "GitHub"
        </a>
      </nav>
    }
}

use leptos::*;

#[component]
pub fn Projects() -> impl IntoView {
    view! {
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
    }
}

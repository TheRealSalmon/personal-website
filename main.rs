use leptos::*;

fn main() {
    console_error_panic_hook::set_once();

    leptos::mount_to_body(|| view! { <App/> })
}

#[component]
pub fn app() -> impl IntoView {
    let (name, set_name) = create_signal("".to_string());

    view! {
      <input
        type="text"
        on:input=move |ev| {
            set_name(event_target_value(&ev));
        }

        prop:value=name
        prop:placeholder="Name"
      />
      <p>"Name is " {name}</p>
    }
}

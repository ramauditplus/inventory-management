use leptos::prelude::*;

#[component]
pub fn StatCard(
    title: &'static str,
    value: Signal<String>,
) -> impl IntoView {
    view! {
        <div class="rounded-xl border bg-white p-5 shadow-sm">
            <p class="text-sm text-gray-500">
                {title}
            </p>

            <p class="mt-2 text-2xl font-bold">
                {value}
            </p>
        </div>
    }
}
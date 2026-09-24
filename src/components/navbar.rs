use leptos::prelude::*;

#[component]
pub fn Navbar() -> impl IntoView{
    view!{
        <header class="h-16 border-b bg-white px-6 flex item-center">
            <h1 class="text-x1 font-semibold">"Inventory Manager"</h1>
        </header>
    }
}
use leptos::prelude::*;

#[component]
pub fn Sidebar() -> impl IntoView{
    view!{
        <aside class="w-64 min-h-[calc(100vh-4rem)] bg-white border-r p-4">
            <nav class="flex flex-col gap-4">
                <a class="rounded-lg px-4 py-2 hover:bg-gray-100" href="/">"Dashboard"</a>
                <a class="rounded-lg px-4 py-2 hover:bg-gray-100" href="/products">"Products"</a>
                <a class="rounded-lg px-4 py-2 hover:bg-gray-100" href="/inventory">"Inventory"</a>
                <a class="rounded-lg px-4 py-2 hover:bg-gray-100" href="/settings">"Settings"</a>
            </nav>
        </aside>
    }
}
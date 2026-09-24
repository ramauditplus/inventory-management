use leptos::prelude::*;

use crate::components::stat_card::StatCard;
use crate::models::product::Product;

#[component]
pub fn Dashboard(products: RwSignal<Vec<Product>>) -> impl IntoView {
    // let total_products = move || products.get().len();
    let total_products = move || products.with(|items| items.len());
    let low_stock =
        move || products.with(|items| items.iter().filter(|product| product.quantity < 5).count());
    let inventory_value = move || {
        products.with(|items| {
            items
                .iter()
                .map(|product| product.price * product.quantity as f64)
                .sum::<f64>()
        })
    };

    view! {
        <div>
            <div class="mb-6">
                <h1 class="text-2xl font-bold">"Dashboard"</h1>

                <p class="mt-1 text-gray-500">"Overview of your inventory"</p>
            </div>

            <div class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-4">

                <StatCard
                    title="Products"
                    value=Signal::derive(move || total_products().to_string())
                />

                <StatCard title="Low Stock" value=Signal::derive(move || low_stock().to_string()) />

                <StatCard
                    title="Inventory Value"
                    value=Signal::derive(move || { format!("₹{:.2}", inventory_value()) })
                />

                <StatCard title="Categories" value=Signal::derive(|| "3".to_string()) />

            </div>
        </div>
    }
}

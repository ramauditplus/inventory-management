use leptos::prelude::*;

use crate::components::navbar::Navbar;
use crate::components::sidebar::Sidebar;
use crate::models::product::Product;
use crate::pages::dashboard::Dashboard;

#[component]
pub fn App() -> impl IntoView {
    let products = RwSignal::new(vec![
        Product {
            id: 1,
            name: "Laptop".to_string(),
            sku: "LAP-001".to_string(),
            price: 55000.0,
            quantity: 10,
        },
        Product {
            id: 2,
            name: "Keyboard".to_string(),
            sku: "KEY-001".to_string(),
            price: 1500.0,
            quantity: 25,
        },
        Product {
            id: 3,
            name: "Mouse".to_string(),
            sku: "MOU-001".to_string(),
            price: 800.0,
            quantity: 4,
        },
    ]);

    view! {
        <div class="min-h-screen bg-gray-100">
            <Navbar />

            <div class="flex">
                <Sidebar />

                <main class="flex-1 p-6">
                    <Dashboard products=products />
                </main>
            </div>
        </div>
    }
}

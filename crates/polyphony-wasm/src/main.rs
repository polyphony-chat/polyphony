mod app;
mod stores;

use chorus::types::Snowflake;
use chorus::UrlBundle;
use leptos::*;
use leptos_router::*;
use log::*;

use crate::app::Register;
use crate::stores::ChorusStore;

#[component]
fn App() -> impl IntoView {
    let instance_store = ChorusStore::default();
    provide_context(instance_store.instances);
    debug!("Rendering the App view");
    view! {
        <Router>
            <main>
                <Routes>
                    <Route path="/register" view=Register/>
                    <Route path="/u" view=|| view! { "hi" }>
                        <Route path=":id" view=|| view! {"meow"}/>
                    </Route>
                </Routes>
            </main>
        </Router>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    debug!("wasm_logger initialized!");
    debug!("Starting App...");
    leptos::mount_to_body(|| view! { <App/> })
}

/// Tuple of a [`UrlBundle`] and a [`Snowflake`], where the [`Snowflake`] is the ID of the User and
/// the [`UrlBundle`] are the URLs of the Instance.
pub type GlobalIdentifier = (UrlBundle, Snowflake);

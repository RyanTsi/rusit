use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::{
    data_struct::Essay,
    utils::{get_entries, get_essays}
};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="tailwindcss" href="/pkg/rusit.css"/>

        // sets the document title
        <Title text="Welcome to rusit"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    let (value, set_value) = create_signal(0);

    // thanks to https://tailwindcomponents.com/component/blue-buttons-example for the showcase layout
    view! {
        <Title text="Leptos + Tailwindcss"/>
        <main>
            <div class="bg-gradient-to-tl from-blue-800 to-blue-500 text-white font-mono flex flex-col min-h-screen">
                <div class="flex flex-row-reverse flex-wrap m-auto">
                    <button on:click=move |_| set_value.update(|value| *value += 1) class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-700 border-blue-800 text-white">
                        "+"
                    </button>
                    <button class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-800 border-blue-900 text-white">
                        {value}
                    </button>
                    <button on:click=move |_| set_value.update(|value| *value -= 1) class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-700 border-blue-800 text-white">
                        "-"
                    </button>
                </div>
            </div>
        </main>
    }
}

async fn load_data() -> Vec<Essay> {
    let ep = get_entries("res", "md");
    let mut essays: Vec<Essay> = Vec::new();
    for i in ep {
        essays.push(get_essays(&i).unwrap());
    }
    essays
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {

    // logging::log!("{:?}", ep);
    
    // let essay = load_data().await;
    // let essay = create_resource(|| (), |_| async move { load_data().await }).get().unwrap();
    // move || match essay.get();
    view! {
        <Title text="Leptos + Tailwindcss"/>
        <main>
            <Hander/>
            
            <div class="min-h-screen relative w-full">
                <div class="w-full max-w-7xl h-auto mt-32 mb-32 mx-auto sm:px-8 pt-32 pb-32 rounded-3xl bg-gray-50 bg-opacity-10 backdrop-blur-md">
                    // <EssayCardList essays=essays/>
                    <h1>"welcome !"</h1>
                </div>
            </div>

            <Foot/>
        </main>
        <Outlet/>
    }
}


#[component]
pub fn Hander() -> impl IntoView {
    view! {
        <div class="fixed top-0 w-full z-10 shadow-md">
            <nav class="py-2 bg-rose-200 bg-opacity-10 backdrop-blur-md backdrop-filter">
                <div class="container mx-auto flex justify-between items-center">
                    <div class="text-black text-3xl font-bold font-serif"> "Logo" </div>
                    <div class="space-x-8">
                        <a href="/" class="hover:underline text-gray-900 hover:text-gray-400"> "Home" </a>
                        <a href="/about" class="hover:underline text-gray-900 hover:text-gray-400"> "About" </a>
                    </div>
                </div>
            </nav>
        </div>
        <Outlet/>
    }
}

#[component]
pub fn EssayCard(
    essay: Memo<Essay>
) -> impl IntoView {
    view! {
        <div class="mx-8 min-h-32 flex flex-row justify-center">
            <div class="sm:w-48 mt-12"> 
                <p class="font-mono text-gray-400 border-b border-gray-200">
                    { move || essay.get().amb.date } 
                </p>
            </div>

            <div class="pl-8 border-l border-gray-200 flex-auto">
                <div class="my-8 min-h-32 backdrop-blur-md bg-opacity-60 bg-gray-50 shadow-lg rounded-2xl overflow-hidden px-auto max-w-xl hover:bg-rose-50">
                    <h1 class="ml-6 mt-4"> {move || essay.get().amb.title} </h1>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn EssayCardList(essays: Vec<Essay>) -> impl IntoView {
    let (data, _) = create_signal(essays);
    view! {
        <div>
            <For
            each=move || data.get().into_iter().enumerate()
            key=|(_, state)| state.amb.date.clone() // 唯一标识
            children=move |(index, _)| {
                let ee = create_memo(move |_| {
                    data.with(|data| data.get(index).map(|d| d.clone()).unwrap())
                });
                view! {
                    <EssayCard essay=ee/>
                }
            }
            />
        </div>
    }
}

#[component]
pub fn Foot() -> impl IntoView {
    view! {
        <div class="bottom-0 z-10 h-28 w-full shadow-sm bg-gray-50 bg-opacity-80 backdrop-blur-md">
            <h1 class="mx-20"> "this is bottom" </h1>
        </div>
    }
}
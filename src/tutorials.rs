use leptos::*;

use leptos_router::*;

#[component]
pub fn RRR() -> impl IntoView {
    view! {
        <Router>
            <h1 class="text-3xl font-bold underline">RRR</h1>
            <nav>
                <a href="/">"Home"</a>
                <a href="/tutorials"> "tutorials "</a>
            </nav>
            <main>
                <Routes>
                    <Route path = "/" view = || view! { <h3>"Ciallo～(∠・ω< )⌒☆"</h3> } />
                    <Route path = "/tutorials" view = || view! {<Outlet/>}>
                        <Route path = ":id" view = PerInfo>
                            <Route path="" view = || view! { 
                                <div class="tab">
                                    "(Contact Info)"
                                </div>}
                            />
                        </Route>
                        <Route path="" view=|| view! {
                            <div>
                                <App/>
                                "Select a user to view contact info."
                            </div>
                        }/>
                    </Route>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
pub fn PerInfo() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.with(|params| params.get("id").cloned().unwrap_or_default());
    let name = move || match id().as_str() {
        "alice" => "Alice",
        "bob" => "Bob",
        "steve" => "Steve",
        _ => "User not found.",
    };
    view! {
        <h2> {name} </h2>
        <Outlet/>
    }
}


#[component]
pub fn App() -> impl IntoView {
    let (cnt, set_cnt) = create_signal(0);
    view! {
        <h1 class:blue=true> c1 </h1>
        <button on:click= move |_| {
            set_cnt.update(|n| *n = *n * 2 + 1);
        }>
            <p>"click me"</p>
        </button>
        <ProgressBar max=4096  progress=cnt/>

        <h1 class:blue=true> c2 </h1>
        <Celection/>

        <h1 class:blue=true> c3 </h1>
        
        <BetterCelction/>
        
        <h1 class:blue=true> c4 </h1>
        <StaticList/>
        
        <Outlet/> // 声明包含有嵌套的子路由
    }
}

#[component]
pub fn Celection() -> impl IntoView {
    let (value, set_value) = create_signal(0);
    let is_odd = move || value.get() & 1 == 1;
    let message = move || if value.get() > 5 {
        view! { <p> "Big" </p> }
    } else {
        view! { <p> "Small" </p> }
    };
    view! {
        <h2> { move || { if is_odd() { "ODD" } else { "EVEN"  } } } </h2>
        {message}
        <button on:click= move|_| {
            set_value.update(|n| *n += 1);
        }>
            "add one"
        </button>
    }
}


#[component]
pub fn BetterCelction() -> impl IntoView {
    let (value, set_value) = create_signal(0);
    view! {
        <Show
            when= move || { value.get() > 5 }
            fallback= || view! {
                <p> "ss"  </p>
            }
        >
        <p>"bb"</p>
        </Show>

        <button on:click = move |_| {
            set_value.update(|n| *n += 1);
        }>
            "add one"
        </button>
    }
}


#[component]
pub fn ProgressBar (
    #[prop(default = 100)]
    max: u16,
    #[prop(into)]
    progress: Signal<i32>
) -> impl IntoView
{
    view! {
        <progress
            max=max
            value=progress
        />
    }
}

#[component]
pub fn StaticList() -> impl IntoView {
    let values = vec![0, 1, 2];
    view! {
        // this will just render "012"
        <p>{values.clone()}</p>
        // or we can wrap them in <li>
        <ul>
            {values.into_iter()
                .map(|n| view! { <li>{n}</li>})
                .collect_view()}
        </ul>
    }
}
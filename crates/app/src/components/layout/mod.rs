use leptos::prelude::*;

// ---------------------------------------------------------------------------
// NavItem shared struct
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub struct NavItem {
    pub label: String,
    pub href: String,
    pub icon: String,
}

// ---------------------------------------------------------------------------
// AppShell
// ---------------------------------------------------------------------------

#[component]
pub fn AppShell(
    #[prop(optional)] sidebar: Option<Children>,
    #[prop(optional)] topbar: Option<Children>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="min-h-screen flex flex-col bg-gray-50">
            {topbar.map(|tb| view! { <header>{tb()}</header> })}
            <div class="flex flex-1 overflow-hidden">
                {sidebar.map(|sb| {
                    view! {
                        <aside class="hidden md:flex md:w-64 md:flex-col border-r border-gray-200 bg-white">
                            {sb()}
                        </aside>
                    }
                })}
                <main class="flex-1 overflow-y-auto p-4 md:p-6">
                    {children()}
                </main>
            </div>
        </div>
    }
}

// ---------------------------------------------------------------------------
// SideNav
// ---------------------------------------------------------------------------

#[component]
pub fn SideNav(
    #[prop(into)] items: Vec<NavItem>,
    #[prop(into, optional)] active_path: String,
) -> impl IntoView {
    view! {
        <nav class="flex flex-col gap-1 p-4">
            {items
                .into_iter()
                .map(|item| {
                    let is_active = item.href == active_path;
                    let cls = if is_active {
                        "flex items-center gap-3 px-3 py-2 rounded-lg bg-blue-50 \
                         text-blue-700 font-medium"
                    } else {
                        "flex items-center gap-3 px-3 py-2 rounded-lg text-gray-600 \
                         hover:bg-gray-100 hover:text-gray-900"
                    };
                    view! {
                        <a href=item.href class=cls>
                            <span class="text-lg">{item.icon}</span>
                            <span>{item.label}</span>
                        </a>
                    }
                })
                .collect_view()}
        </nav>
    }
}

// ---------------------------------------------------------------------------
// TopBar
// ---------------------------------------------------------------------------

#[component]
pub fn TopBar(
    #[prop(into, optional)] title: String,
    #[prop(into, optional)] user_name: String,
    #[prop(into, optional)] on_logout: Option<Callback<()>>,
) -> impl IntoView {
    view! {
        <div class="flex items-center justify-between h-16 px-4 md:px-6 \
                    bg-white border-b border-gray-200">
            <h1 class="text-lg font-semibold text-gray-800">{title}</h1>
            <div class="flex items-center gap-4">
                {if !user_name.is_empty() {
                    Some(view! {
                        <span class="text-sm text-gray-600">{user_name}</span>
                    })
                } else {
                    None
                }}
                {on_logout.map(|cb| {
                    view! {
                        <button
                            class="text-sm text-gray-500 hover:text-gray-700"
                            on:click=move |_| cb.run(())
                        >
                            "로그아웃"
                        </button>
                    }
                })}
            </div>
        </div>
    }
}

// ---------------------------------------------------------------------------
// BottomNavBar
// ---------------------------------------------------------------------------

#[component]
pub fn BottomNavBar(
    #[prop(into)] items: Vec<NavItem>,
    #[prop(into, optional)] active_path: String,
) -> impl IntoView {
    view! {
        <nav class="fixed bottom-0 inset-x-0 z-40 bg-white border-t border-gray-200 \
                    md:hidden safe-bottom">
            <div class="flex items-center justify-around h-16">
                {items
                    .into_iter()
                    .map(|item| {
                        let is_active = item.href == active_path;
                        let color = if is_active { "text-blue-600" } else { "text-gray-400" };
                        let font = if is_active { "font-medium" } else { "" };
                        let cls = format!(
                            "flex flex-col items-center justify-center gap-0.5 \
                             min-w-[4rem] {color} {font}"
                        );
                        view! {
                            <a href=item.href class=cls>
                                <span class="text-xl">{item.icon}</span>
                                <span class="text-[10px]">{item.label}</span>
                            </a>
                        }
                    })
                    .collect_view()}
            </div>
        </nav>
    }
}

// ---------------------------------------------------------------------------
// PageHeader
// ---------------------------------------------------------------------------

#[component]
pub fn PageHeader(
    #[prop(into)] title: String,
    #[prop(into, optional)] subtitle: String,
    #[prop(optional)] action: Option<Children>,
) -> impl IntoView {
    view! {
        <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-2 mb-6">
            <div>
                <h2 class="text-2xl font-bold text-gray-900">{title}</h2>
                {if !subtitle.is_empty() {
                    Some(view! {
                        <p class="mt-1 text-sm text-gray-500">{subtitle}</p>
                    })
                } else {
                    None
                }}
            </div>
            {action.map(|a| view! { <div>{a()}</div> })}
        </div>
    }
}

// ---------------------------------------------------------------------------
// CardGrid
// ---------------------------------------------------------------------------

#[component]
pub fn CardGrid(
    #[prop(optional, default = 3)] cols: u8,
    children: Children,
) -> impl IntoView {
    let grid_cls = match cols {
        1 => "grid grid-cols-1 gap-4",
        2 => "grid grid-cols-1 sm:grid-cols-2 gap-4",
        3 => "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4",
        _ => "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4",
    };

    view! {
        <div class=grid_cls>
            {children()}
        </div>
    }
}

// ---------------------------------------------------------------------------
// Section
// ---------------------------------------------------------------------------

#[component]
pub fn Section(
    #[prop(into, optional)] title: String,
    children: Children,
) -> impl IntoView {
    view! {
        <section class="mb-8">
            {if !title.is_empty() {
                Some(view! {
                    <h3 class="text-lg font-semibold text-gray-800 mb-4">{title}</h3>
                })
            } else {
                None
            }}
            {children()}
        </section>
    }
}

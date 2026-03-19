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

/// Top-level application shell that composes a sidebar, top bar, and main
/// content area into a responsive layout. The sidebar is hidden on mobile
/// and the main content region scrolls independently.
#[component]
pub fn AppShell(
    #[prop(optional)] sidebar: Option<Children>,
    #[prop(optional)] topbar: Option<Children>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="min-h-screen flex flex-col bg-surface-page">
            {topbar.map(|tb| view! { <header>{tb()}</header> })}
            <div class="flex flex-1 overflow-hidden">
                {sidebar.map(|sb| {
                    view! {
                        <aside class="hidden md:flex md:w-64 md:flex-col \
                                      shadow-[2px_0_8px_rgba(0,0,0,0.04)] bg-white">
                            {sb()}
                        </aside>
                    }
                })}
                <main class="flex-1 overflow-y-auto p-4 md:p-6 animate-fade-in">
                    {children()}
                </main>
            </div>
        </div>
    }
}

// ---------------------------------------------------------------------------
// SideNav
// ---------------------------------------------------------------------------

/// Vertical navigation rendered inside the sidebar. Each item is styled with
/// the portal's accent colour when active, using CSS custom properties
/// `--portal-accent` and `--portal-accent-light`.
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
                        "flex items-center gap-3 px-3 py-2 \
                         bg-[var(--portal-accent-light)] text-[var(--portal-accent)] \
                         rounded-xl font-semibold"
                    } else {
                        "flex items-center gap-3 px-3 py-2 \
                         text-txt-secondary hover:bg-surface-subtle \
                         hover:text-txt-primary rounded-xl"
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

/// Horizontal top bar showing the page title, optional user name, and a
/// logout button. Uses a subtle box-shadow instead of a border for
/// separation from the content area below.
#[component]
pub fn TopBar(
    #[prop(into, optional)] title: String,
    #[prop(into, optional)] user_name: String,
    #[prop(into, optional)] on_logout: Option<Callback<()>>,
) -> impl IntoView {
    view! {
        <div class="flex items-center justify-between h-16 px-4 md:px-6 \
                    bg-white shadow-sm">
            <h1 class="text-lg font-semibold text-txt-primary">{title}</h1>
            <div class="flex items-center gap-4">
                {if !user_name.is_empty() {
                    Some(view! {
                        <span class="text-sm text-txt-secondary">{user_name}</span>
                    })
                } else {
                    None
                }}
                {on_logout.map(|cb| {
                    view! {
                        <button
                            class="text-sm text-txt-tertiary hover:text-txt-primary"
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

/// Fixed bottom navigation bar visible only on mobile. The active tab uses
/// the portal accent colour with a small dot indicator beneath the label.
#[component]
pub fn BottomNavBar(
    #[prop(into)] items: Vec<NavItem>,
    #[prop(into, optional)] active_path: String,
) -> impl IntoView {
    view! {
        <nav class="fixed bottom-0 inset-x-0 z-40 bg-white \
                    shadow-[0_-2px_8px_rgba(0,0,0,0.06)] md:hidden safe-bottom">
            <div class="flex items-center justify-around h-16">
                {items
                    .into_iter()
                    .map(|item| {
                        let is_active = item.href == active_path;
                        let color = if is_active {
                            "text-[var(--portal-accent)]"
                        } else {
                            "text-txt-disabled"
                        };
                        let font = if is_active { "font-medium" } else { "" };
                        let cls = format!(
                            "flex flex-col items-center justify-center gap-0.5 \
                             min-w-[4rem] {color} {font}"
                        );
                        view! {
                            <a href=item.href class=cls>
                                <span class="text-xl">{item.icon}</span>
                                <span class="text-[10px]">{item.label}</span>
                                {if is_active {
                                    Some(view! {
                                        <span class="block w-1 h-1 rounded-full \
                                                     bg-[var(--portal-accent)] mt-0.5"></span>
                                    })
                                } else {
                                    None
                                }}
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

/// Page-level header with a title, optional subtitle, and an optional
/// trailing action slot (e.g. a button). Uses generous bottom margin to
/// separate it from the content below.
#[component]
pub fn PageHeader(
    #[prop(into)] title: String,
    #[prop(into, optional)] subtitle: String,
    #[prop(optional)] action: Option<Children>,
) -> impl IntoView {
    view! {
        <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-2 mb-8">
            <div>
                <h2 class="text-2xl font-bold text-txt-primary">{title}</h2>
                {if !subtitle.is_empty() {
                    Some(view! {
                        <p class="mt-1 text-sm text-txt-tertiary">{subtitle}</p>
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

/// Responsive grid container for cards. Accepts a `cols` hint (1-4) and
/// renders a CSS grid with wider `gap-6` spacing.
#[component]
pub fn CardGrid(
    #[prop(optional, default = 3)] cols: u8,
    children: Children,
) -> impl IntoView {
    let grid_cls = match cols {
        1 => "grid grid-cols-1 gap-6",
        2 => "grid grid-cols-1 sm:grid-cols-2 gap-6",
        3 => "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6",
        _ => "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6",
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

/// Generic content section with an optional title and generous bottom margin
/// for visual separation between sections.
#[component]
pub fn Section(
    #[prop(into, optional)] title: String,
    children: Children,
) -> impl IntoView {
    view! {
        <section class="mb-10">
            {if !title.is_empty() {
                Some(view! {
                    <h3 class="text-lg font-semibold text-txt-primary mb-4">{title}</h3>
                })
            } else {
                None
            }}
            {children()}
        </section>
    }
}

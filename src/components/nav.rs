use dioxus::prelude::*;
use crate::route_enum::Route;

#[component]
pub fn Nav() -> Element {
    let route = use_route::<Route>();
    let cv_active = matches!(route, Route::Home {});
    let articles_active = matches!(route, Route::Articles {} | Route::PostDetail { .. });

    rsx! {
        nav { class: "nav",
            ul { class: "nav__list",
                li { class: "nav__item",
                    Link {
                        class: if cv_active { "nav__link nav__link--active" } else { "nav__link" },
                        to: Route::Home {},
                        "CV"
                    }
                }
                li { class: "nav__item",
                    Link {
                        class: if articles_active { "nav__link nav__link--active" } else { "nav__link" },
                        to: Route::Articles {},
                        "Articles"
                    }
                }
            }
        }
    }
}

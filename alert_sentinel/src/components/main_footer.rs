use dioxus::prelude::*;
use dioxus_i18n::t;

/// MainFooter component displays the application footer.
///
/// # Props
/// - `running`: Signal indicating whether monitoring is active.
///
/// The footer shows the current monitoring status, copyright,
/// version, and a link to the GitHub repository.
#[component]
pub fn MainFooter(running: Signal<bool>) -> Element {
    rsx! {
        footer { class: "footer sm:footer-horizontal bg-neutral text-neutral-content items-center p-4",
            // Monitoring status badge
            if running() {
                div { class: "grid-flow-col items-center badge badge-neutral opacity-70",
                    svg {
                        class: "text-accent",
                        xmlns: "http://www.w3.org/2000/svg",
                        width: "18",
                        height: "18",
                        view_box: "0 0 16 16",
                        path {
                            fill: "currentColor",
                            d: "M8 6.003a2.667 2.667 0 1 1 0 5.334a2.667 2.667 0 0 1 0-5.334m0 1a1.667 1.667 0 1 0 0 3.334a1.667 1.667 0 0 0 0-3.334m0-3.336c3.076 0 5.73 2.1 6.467 5.043a.5.5 0 1 1-.97.242a5.67 5.67 0 0 0-10.995.004a.5.5 0 0 1-.97-.243A6.67 6.67 0 0 1 8 3.667",
                        }
                    }
                    p { class: "text-accent", {t!("footer_monitoring")} }
                }
            } else {
                div { class: "grid-flow-col items-center badge badge-neutral opacity-70",
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        width: "18",
                        height: "18",
                        view_box: "0 0 16 16",
                        path {
                            fill: "currentColor",
                            d: "M1.48 1.48a.5.5 0 0 0-.049.65l.049.057l2.69 2.69A6.66 6.66 0 0 0 1.533 8.71a.5.5 0 0 0 .97.242a5.66 5.66 0 0 1 2.386-3.356l1.207 1.207a2.667 2.667 0 0 0 3.771 3.771l3.946 3.946a.5.5 0 0 0 .756-.65l-.049-.057l-4.075-4.076v-.001l-.8-.799l-1.913-1.913h.001l-1.92-1.919v-.001l-.755-.754l-2.871-2.87a.5.5 0 0 0-.707 0m5.323 6.03l2.356 2.357A1.667 1.667 0 0 1 6.802 7.51M8 3.667c-.667 0-1.314.098-1.926.283l.825.824Q7.435 4.668 8 4.667a5.67 5.67 0 0 1 5.498 4.288a.5.5 0 0 0 .97-.242A6.67 6.67 0 0 0 8 3.667m.13 2.34l2.534 2.533A2.67 2.67 0 0 0 8.13 6.006",
                        }
                    }
                    {t!("footer_stopped")}
                }
            }
            // Copyright information
            div { class: "md:place-self-center text-sm text-center opacity-70", "© 2025 st-little" }
            // Version and GitHub link
            div { class: "grid-flow-col gap-4 md:place-self-center md:justify-self-end opacity-70",
                p { "v0.1.0" }
                a {
                    class: "mt-[2px]",
                    href: "https://github.com/st-little/rust-screen-suite",
                    target: "_blank",
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        width: "16",
                        height: "16",
                        view_box: "0 0 16 16",
                        path {
                            fill: "currentColor",
                            fill_rule: "evenodd",
                            d: "M7.976 0A7.977 7.977 0 0 0 0 7.976c0 3.522 2.3 6.507 5.431 7.584c.392.049.538-.196.538-.392v-1.37c-2.201.49-2.69-1.076-2.69-1.076c-.343-.93-.881-1.175-.881-1.175c-.734-.489.048-.489.048-.489c.783.049 1.224.832 1.224.832c.734 1.223 1.859.88 2.3.685c.048-.538.293-.88.489-1.076c-1.762-.196-3.621-.881-3.621-3.964c0-.88.293-1.566.832-2.153c-.05-.147-.343-.978.098-2.055c0 0 .685-.196 2.201.832c.636-.196 1.322-.245 2.007-.245s1.37.098 2.006.245c1.517-1.027 2.202-.832 2.202-.832c.44 1.077.146 1.908.097 2.104a3.16 3.16 0 0 1 .832 2.153c0 3.083-1.86 3.719-3.62 3.915c.293.244.538.733.538 1.467v2.202c0 .196.146.44.538.392A7.98 7.98 0 0 0 16 7.976C15.951 3.572 12.38 0 7.976 0",
                            clip_rule: "evenodd",
                        }
                    }

                }
            }
        }
    }
}

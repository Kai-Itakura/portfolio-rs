use topcoat::{
    Result,
    asset::{Asset, asset},
    router::href,
    view::{View, component, view},
};

use crate::{app, site};

const LOGO: Asset = asset!("assets/logo.png");

#[component]
pub(crate) async fn header() -> Result<impl View> {
    Ok(view! {
        <header
            class="group relative z-3 flex h-14 items-center gap-6.5 \
                   border-b border-border px-[max(18px,3vw)] max-sm:h-13 max-sm:gap-3"
        >
            <span class="flex items-center gap-2.25 text-sm font-semibold whitespace-nowrap">
                <img class="size-5.5" src=(LOGO) alt="" width="22" height="22" />
                " Kai Itakura"
            </span>

            <ul class="flex gap-5.5 text-[13.5px] text-muted max-md:hidden">
                <li>
                    <a class="hover:text-fg aria-[current]:text-fg" href=(href!(app::home))>
                        "Home"
                    </a>
                </li>
                <li>
                    <a class="hover:text-fg aria-[current]:text-fg" href="/works">"Works"</a>
                </li>
                <li>
                    <a
                        class="hover:text-fg aria-[current]:text-fg"
                        href=(href!(app::about::about))
                    >
                        "About"
                    </a>
                </li>
            </ul>

            <span class="ml-auto flex items-center gap-2.5">
                <a
                    class="inline-flex h-8 items-center gap-1.5 rounded-btn border \
                           border-border-strong px-3.25 text-[13.5px] font-medium \
                           whitespace-nowrap text-fg transition-colors duration-150 \
                           hover:border-muted hover:bg-card max-md:hidden"
                    href=(site::GITHUB_URL)
                    target="_blank"
                    rel="noopener noreferrer"
                >
                    "GitHub"
                </a>
                <a
                    class="inline-flex h-8 items-center gap-1.5 rounded-btn bg-fg px-3.25 \
                           text-[13.5px] font-medium whitespace-nowrap text-bg \
                           transition-opacity duration-150 hover:opacity-88"
                    href="#contact"
                >
                    "Contact"
                </a>
                <label
                    class="hidden size-9 cursor-pointer flex-col items-center justify-center \
                           gap-1 rounded-lg border border-border-strong max-md:flex"
                >
                    <input class="absolute size-0 opacity-0" type="checkbox" />
                    <i
                        class="block h-[1.5px] w-3.75 rounded-xs bg-fg transition-transform \
                               duration-200 group-has-checked:translate-y-[2.75px] \
                               group-has-checked:rotate-45"
                    ></i>
                    <i
                        class="block h-[1.5px] w-3.75 rounded-xs bg-fg transition-transform \
                               duration-200 group-has-checked:-translate-y-[2.75px] \
                               group-has-checked:-rotate-45"
                    ></i>
                    <span class="sr-only">"メニュー"</span>
                </label>
            </span>

            <div
                class="absolute inset-x-0 top-full z-4 hidden gap-0.5 border-b border-border \
                       bg-bg px-[max(18px,3vw)] pt-2.5 pb-4 group-has-checked:grid"
            >
                <a
                    class="border-b border-border px-0.5 py-2.75 text-[15px]"
                    href=(href!(app::home))
                >
                    "Home"
                </a>
                <a class="border-b border-border px-0.5 py-2.75 text-[15px]" href="/works">
                    "Works"
                </a>
                <a
                    class="border-b border-border px-0.5 py-2.75 text-[15px]"
                    href=(href!(app::about::about))
                >
                    "About"
                </a>
                <a
                    class="mt-2.5 inline-flex h-8 items-center justify-center gap-1.5 \
                           rounded-btn bg-fg px-3.25 text-[13.5px] font-medium \
                           whitespace-nowrap text-bg transition-opacity duration-150 \
                           hover:opacity-88"
                    href="#contact"
                >
                    "Contact"
                </a>
            </div>
        </header>
    })
}

use topcoat::{
    Result,
    icon::{IconData, icon, iconify},
    view::{View, component, view},
};

const ARROW_UP_RIGHT: IconData = iconify::iconify_icon!("lucide:arrow-up-right");

#[component]
pub(crate) async fn work_card(
    href: &str,
    path: &str,
    title: &str,
    description: &str,
    tags: &[&str],
) -> Result<impl View> {
    Ok(view! {
        <a
            class="group block min-w-0 overflow-hidden rounded-card border border-border bg-card \
                   transition-[border-color,box-shadow] duration-180 hover:border-border-strong \
                   hover:shadow-[0_12px_30px_-16px] hover:shadow-accent/40"
            href=(href)
        >
            <span
                class="block aspect-16/10 border-b border-border bg-bg opacity-92 \
                       transition-opacity duration-180 group-hover:opacity-100 max-sm:aspect-video"
                aria-hidden="true"
            ></span>
            <div class="px-4 pt-4 pb-4.25">
                <span
                    class="mb-1.25 block font-mono text-[10.5px] text-accent opacity-85"
                >
                    (path)
                </span>
                <div class="flex items-center gap-2">
                    <h3 class="text-[14.5px] font-medium tracking-[-0.02em]">
                        (title)
                    </h3>
                    <span
                        class="ml-auto inline-flex shrink-0 text-muted \
                               transition-[translate,color] duration-180 \
                               group-hover:translate-x-0.5 group-hover:-translate-y-0.5 \
                               group-hover:text-fg"
                    >
                        icon(data: ARROW_UP_RIGHT, size: 15)
                    </span>
                </div>
                <span class="mt-1.5 block text-[13px] tracking-normal text-muted">
                    (description)
                </span>
                <span class="mt-3.5 flex flex-wrap gap-1.25">
                    for tag in tags {
                        <span
                            class="rounded-btn border border-border bg-bg px-1.75 py-0.5 \
                                   font-mono text-[10.5px] text-muted"
                        >
                            (*tag)
                        </span>
                    }
                </span>
            </div>
        </a>
    })
}

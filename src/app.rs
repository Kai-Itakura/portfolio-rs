pub(crate) mod about;

use topcoat::{
    Result,
    asset::{AssetBundle, RouterBuilderAssetExt},
    icon::{IconData, icon, iconify},
    router::{Router, RouterBuilderDiscoverExt, href, page},
    view::{View, component, view},
};

use crate::components::{base_layout::base_layout, work_card::work_card};

const CHEVRON_RIGHT: IconData = iconify::iconify_icon!("lucide:chevron-right");

pub fn router() -> Router {
    topcoat::router::module_router!()
        .discover()
        .assets(AssetBundle::load().unwrap())
        .build()
}

#[page]
pub(crate) async fn home() -> Result<impl View> {
    Ok(view! {
        base_layout(
            hero()
            works_section()
        )
    })
}

#[component]
async fn hero() -> Result<impl View> {
    Ok(view! {
        <section
            class="relative isolate overflow-hidden border-b border-border \
                   px-[max(18px,3vw)] pt-[max(42px,5.4vw)] pb-[max(38px,5vw)] \
                   max-sm:pt-8.5 max-sm:pb-8"
        >
            <div
                class="pointer-events-none absolute inset-0 -z-10 \
                       bg-[radial-gradient(circle,rgb(255_255_255/0.07)_1px,transparent_1px)] \
                       bg-size-[22px_22px] \
                       mask-[radial-gradient(ellipse_70%_60%_at_30%_0%,#000_20%,transparent_75%)]"
                aria-hidden="true"
            ></div>
            <div
                class="pointer-events-none absolute top-[-60%] left-[26%] -z-10 aspect-2/1 \
                       w-[62%] -translate-x-1/2 bg-radial from-accent/17 via-accent/4 via-42% \
                       to-transparent to-68%"
                aria-hidden="true"
            ></div>

            <p class="flex gap-2.25 font-mono text-[13px] text-muted">
                <span class="text-accent">"$"</span>
                "whoami"
            </p>
            <h1
                class="mt-4 max-w-[20ch] font-mono text-[clamp(24px,4.4vw,50px)]/[1.1] \
                       font-medium tracking-[-0.03em] max-sm:mt-3.5 max-sm:max-w-[16ch] \
                       max-sm:text-[clamp(22px,7.4vw,30px)]"
            >
                "Ship it,"
                <br />
                "then grow it."
            </h1>
            <p
                class="mt-4.5 max-w-[46ch] text-[15px]/[1.8] tracking-normal text-muted \
                       max-sm:mt-4 max-sm:text-[14px]/[1.85]"
            >
                "受託と SES で、フロントエンドからバックエンドまで 3 年。設計から運用まで一通り通してきました。\
                 これからは自社プロダクトを新規開発から運用まで、長く育てていきたいと考えています。"
            </p>
            <dl
                class="mt-6.5 grid max-w-[56ch] grid-cols-[auto_minmax(0,1fr)] gap-x-5.5 \
                       gap-y-0.5 font-mono text-[12px] max-sm:mt-5 \
                       max-sm:grid-cols-[72px_minmax(0,1fr)] max-sm:gap-x-3 max-sm:gap-y-0.75 \
                       max-sm:text-[11.5px]"
            >
                <dt class="text-muted">"base"</dt>
                <dd class="font-sans tracking-normal">"東京 / 石川出身"</dd>
                <dt class="text-muted">"stack"</dt>
                <dd>"Go · TypeScript · React · AWS · Docker"</dd>
                <dt class="text-muted">"status"</dt>
                <dd class="font-sans tracking-normal">
                    "求職中 — Web アプリケーション / バックエンド / フロントエンド"
                </dd>
                <dt class="text-muted">"this site"</dt>
                <dd>"Rust + Topcoat 0.9.0 on Cloud Run"</dd>
            </dl>
            <div class="mt-6.5 flex flex-wrap gap-2.5 max-sm:mt-5.5 max-sm:gap-2">
                <a
                    class="inline-flex h-10 items-center gap-1.5 rounded-btn bg-fg px-4.5 \
                           text-[14.5px] font-medium tracking-normal whitespace-nowrap text-bg \
                           transition-opacity duration-150 hover:opacity-88 \
                           max-sm:flex-auto max-sm:justify-center"
                    href="/works"
                >
                    "作品を見る"
                </a>
                <a
                    class="inline-flex h-10 items-center gap-1.5 rounded-btn border \
                           border-border-strong px-4.5 text-[14.5px] font-medium tracking-normal \
                           whitespace-nowrap text-fg transition-colors duration-150 \
                           hover:border-muted hover:bg-card max-sm:flex-auto max-sm:justify-center"
                    href=(href!(about::about))
                >
                    "経歴を読む"
                </a>
            </div>
        </section>
    })
}

#[component]
async fn works_section() -> Result<impl View> {
    Ok(view! {
        <section class="px-[max(18px,3vw)] py-[max(34px,4.4vw)]">
            <div class="mb-5.5 flex items-end gap-4 max-sm:mb-4">
                <div>
                    <h2
                        class="text-[clamp(19px,2.2vw,26px)] font-semibold tracking-[-0.035em]"
                    >
                        "Works"
                    </h2>
                    <p class="mt-1 text-[13.5px] tracking-normal text-muted">
                        "個人で作ったものです。実務の案件は About の Timeline に書いています。"
                    </p>
                </div>
                <a
                    class="ml-auto flex items-center gap-1.25 text-[13.5px] tracking-normal \
                           whitespace-nowrap text-muted hover:text-fg max-sm:text-[12.5px]"
                    href="/works"
                >
                    "すべて見る"
                    icon(data: CHEVRON_RIGHT, size: 13)
                </a>
            </div>
            <div class="grid grid-cols-3 gap-4 max-md:grid-cols-1">
                work_card(
                    href: "/works/portfolio",
                    path: "works/portfolio",
                    title: "Kai Itakura | Portfolio",
                    description: "Jamstack 構成のポートフォリオ。microCMS の内容を静的生成で配信。",
                    tags: &["Next.js", "TypeScript", "Sass", "microCMS"]
                )
                work_card(
                    href: "/works/sixhelmets",
                    path: "works/sixhelmets",
                    title: "sixhelmets.co.,ltd.",
                    description: "アパレルブランドのコーポレートサイト。XD で設計し、素の JS で実装。",
                    tags: &["JavaScript", "Sass", "XD", "Illustrator"]
                )
                work_card(
                    href: "/works/sugutabe",
                    path: "works/sugutabe",
                    title: "スグ食べ",
                    description: "オーガニック食材の LP。ヒアリングから導線設計まで担当。",
                    tags: &["jQuery", "HTML5", "CSS3", "Illustrator"]
                )
            </div>
        </section>
    })
}

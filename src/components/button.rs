use topcoat::view::{Class, ClassEntries, class};

const BASE_CLASS: &str =
    "inline-flex items-center gap-1.5 rounded-btn font-medium whitespace-nowrap duration-150";

#[derive(Copy, Clone)]
pub(crate) enum ButtonVariant {
    Primary,
    Ghost,
}

impl ButtonVariant {
    fn classes(self) -> &'static str {
        match self {
            Self::Primary => "bg-fg text-bg transition-opacity hover:opacity-88",
            Self::Ghost => {
                "border border-border-strong text-fg transition-colors hover:border-muted hover:bg-card"
            }
        }
    }
}

#[derive(Copy, Clone)]
pub(crate) enum ButtonSize {
    Small,
    Large,
}

impl ButtonSize {
    fn classes(self) -> &'static str {
        match self {
            Self::Small => "h-8 px-3.25 text-[13.5px]",
            Self::Large => "h-10 px-4.5 text-[14.5px]",
        }
    }
}

pub(crate) fn button_variants(
    variant: ButtonVariant,
    size: ButtonSize,
) -> Class<impl ClassEntries> {
    class!(BASE_CLASS, variant.classes(), size.classes())
}

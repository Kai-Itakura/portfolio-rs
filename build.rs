fn main() {
    topcoat::tailwind::BuildConfig::new()
        .input("app.css")
        .render()
        .unwrap();

    topcoat::icon::iconify::BuildConfig::new()
        .icon_set("lucide")
        .stage()
        .unwrap();
}

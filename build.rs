fn main() {
    slint_build::compile("ui/main.slint").expect("Slint build failed");
    let mut res = winres::WindowsResource::new();
    res.set_icon("ui/imgs/app.ico");
    res.compile().expect("Failed to compile resources");
}

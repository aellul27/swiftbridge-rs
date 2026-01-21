use appkit::app::App;

fn main() {
    let app = match App::create() {
        Ok(a) => a,
        Err(e) => {
            eprintln!("Failed to create app: {}", e);
            return;
        }
    };

    let _window = match app.create_window(200.0, 200.0, 640.0, 420.0, "Single Window") {
        Ok(w) => w,
        Err(e) => {
            eprintln!("Failed to create window: {}", e);
            return;
        }
    };
    app.run();
}
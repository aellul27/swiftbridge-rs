use std::sync::Arc;
use std::thread;
use std::time::Duration;

use appkit::app::App;

fn main() {
    let app = match App::create() {
        Ok(a) => a,
        Err(e) => {
            eprintln!("Failed to create app: {}", e);
            return;
        }
    };

    let app = Arc::new(app);
    let app_for_thread = Arc::clone(&app);

    thread::spawn(move || {
        let window = match app_for_thread.create_window(
            200.0,
            200.0,
            640.0,
            420.0,
            "Window 1",
        ) {
            Ok(w) => w,
            Err(e) => {
                eprintln!("Failed to create window: {}", e);
                return;
            }
        };

        let window2 = match app_for_thread.create_window(
            200.0,
            200.0,
            640.0,
            420.0,
            "Window 2",
        ) {
            Ok(w) => w,
            Err(e) => {
                eprintln!("Failed to create window: {}", e);
                return;
            }
        };
        app_for_thread.activate();
        thread::sleep(Duration::from_secs(5));
        window.destroy();
        window2.destroy();
        app_for_thread.stop();
        thread::park();
    });

    app.run();
    app.terminate()
}
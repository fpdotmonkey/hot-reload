use std::os::raw::c_char;

use argh::FromArgs;
use common::{FrameContext, Pixel};
use minifb::{Key, Window, WindowOptions};
use notify::Watcher;

mod plugin;

use plugin::Plugin;

compromise::register!(); // set up hot reloading

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Args = argh::from_env();
    compromise::set_hot_reload_enabled(args.watch);
    if args.watch {
        println!("Hot reloading enabled - there will be memory leaks!");
    }

    let base = std::path::PathBuf::from("../libgreet-rs")
        .canonicalize()
        .unwrap();
    let libname = "libgreet.so";
    let relative_path = std::path::PathBuf::from("target")
        .join("debug")
        .join(libname);
    let absolute_path = base.join(&relative_path);

    // communication between the watcher thread and the main thread
    let (tx, rx) = std::sync::mpsc::channel::<()>();

    let mut watcher: notify::RecommendedWatcher = notify::Watcher::new(
        {
            move |res: Result<notify::Event, _>| match res {
                Ok(event) => {
                    if let notify::EventKind::Create(_) = event.kind {
                        if event.paths.iter().any(|x| x.ends_with(&relative_path)) && args.watch {
                            // signal to reload the library
                            tx.send(()).unwrap();
                        }
                    }
                }
                Err(err) => println!("watch error: {}", err),
            }
        },
        notify::Config::default(),
    )
    .unwrap();
    watcher
        .watch(&base, notify::RecursiveMode::Recursive)
        .unwrap();

    const WIDTH: usize = 640;
    const HEIGHT: usize = 360;
    let mut pixels: Vec<Pixel> = Vec::with_capacity(WIDTH * HEIGHT);
    for _ in 0..pixels.capacity() {
        pixels.push(Pixel {
            b: 0,
            g: 0,
            r: 0,
            z: 0,
        });
    }

    let mut window = Window::new("Playground", WIDTH, HEIGHT, WindowOptions::default())?;
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut plugin = Some(Plugin::load(&absolute_path).unwrap());
    let start = std::time::SystemTime::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if rx.try_recv().is_ok() {
            println!("==== Reloading ====");
            plugin = None; // causes the previous plugin to be dropped,
                           // which will call dlclose.  Without this,
                           // you'd just be increasing the library's
                           // refcount.
            plugin = Some(Plugin::load(&absolute_path)?);
        }

        if let Some(plugin) = plugin.as_ref() {
            let mut context = FrameContext {
                width: WIDTH,
                height: HEIGHT,
                pixels: &mut pixels[0],
                ticks: start.elapsed().unwrap().as_millis() as usize,
            };
            (plugin.draw)(&mut context);
        }

        window
            .update_with_buffer(
                #[allow(clippy::transmute_ptr_to_ptr)]
                unsafe {
                    std::mem::transmute(pixels.as_slice())
                },
                WIDTH,
                HEIGHT,
            )
            .unwrap();
    }

    Ok(())
}

#[derive(FromArgs)]
/// Demonstrate hot reloading in rust
struct Args {
    #[argh(switch, description = "enable hot reloading")]
    watch: bool,
}

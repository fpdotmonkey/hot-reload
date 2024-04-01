use std::os::raw::c_char;

use argh::FromArgs;
use libloading::{Library, Symbol};
use notify::Watcher;

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

    let mut watcher: notify::RecommendedWatcher = notify::Watcher::new(
        {
            move |res: Result<notify::Event, _>| match res {
                Ok(event) => {
                    if let notify::EventKind::Create(_) = event.kind {
                        if event.paths.iter().any(|x| x.ends_with(&relative_path)) {
                            let res = step(&absolute_path);
                            if let Err(e) = res {
                                println!("step error: {}", e);
                            }
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

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

#[derive(FromArgs)]
/// Demonstrate hot reloading in rust
struct Args {
    #[argh(switch, description = "enable hot reloading")]
    watch: bool,
}

fn step(lib_path: &std::path::Path) -> Result<(), libloading::Error> {
    unsafe {
        let lib = Library::new(lib_path)?;
        let greet: Symbol<unsafe extern "C" fn(name: *const c_char)> = lib.get(b"greet")?;
        #[allow(clippy::transmute_ptr_to_ref)]
        greet(c"saturday".as_ptr());
    }

    Ok(())
}

fn run() {
    let mut line = String::new();
    let stdin = std::io::stdin();

    println!("starting up");
    let n = 3;
    for _ in 0..n {
        load_and_print().unwrap();

        println!("------------------");
        println!("Press Enter to reload, ^C to exit");

        line.clear();
        stdin.read_line(&mut line).unwrap();
    }

    println!("Did {n} rounds, stopping");
}

fn load_and_print() -> Result<Library, libloading::Error> {
    unsafe {
        let lib = Library::new("../libgreet-rs/target/debug/libgreet.so")?;
        let greet: Symbol<unsafe extern "C" fn(name: *const c_char)> = lib.get(b"greet")?;
        greet(c"reloading".as_ptr());
        Ok(lib)
    }
}

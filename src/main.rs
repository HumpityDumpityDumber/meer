use netwatcher::watch_interfaces;
use online::check;

enum Predator {
    Online,
    NiriSocket,
}

struct Sentinel {
    name: &'static str,
    predator: Predator,
}

static NEED_NIRI: Sentinel = Sentinel {
    name: "Niri Flagship Desktop",
    predator: Predator::NiriSocket,
};

static NEED_ONLINE: Sentinel = Sentinel {
    name: "Automatic Wallpaper Fetcher",
    predator: Predator::Online,
};

#[tokio::main]
async fn main() {
    let sentinels = [&NEED_ONLINE, &NEED_NIRI];

    for s in sentinels {
        println!("Processing Sentinel: {}", s.name);
        pred_watcher(s).await;
    }
}

async fn pred_watcher(current_sentinel: &Sentinel) {
    match current_sentinel.predator {
        Predator::Online => check_online(),
        Predator::NiriSocket => println!("This doesn't do anything"),
    }
}

fn check_online() {
    if check(None).is_ok() {
        println!("Internet connected")
    } else {
        println!("Internet connection not found... waiting for update to interfaces");
        watch_net_update();
    }
}

fn watch_net_update() {
    let handle = watch_interfaces(|_| {
        println!("Interface update!");
    })
    .unwrap();

    drop(handle);
}

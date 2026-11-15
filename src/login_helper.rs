use std::path::PathBuf;

pub fn run() -> i32 {
    let mut out: Option<PathBuf> = None;
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == "--out" {
            out = args.next().map(PathBuf::from);
        }
    }

    let Some(out) = out else {
        eprintln!("login-helper: --out is required");
        return 2;
    };

    let cookie_src = PathBuf::from("/data/roblox-2016-xbox/cookie.txt");
    if let Ok(cookie) = std::fs::read_to_string(&cookie_src) {
        let trimmed = cookie.trim();
        if !trimmed.is_empty() {
            if std::fs::write(&out, trimmed).is_ok() {
                return 0;
            }
        }
    }

    eprintln!(
        "login-helper (PS4): coloque o cookie .ROBLOSECURITY em {}",
        cookie_src.display()
    );
    1
}

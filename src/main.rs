use rblitz::{config, game_server};

fn main() {
    setup_logger().unwrap();
    unsafe { enet_sys::enet_initialize() };
    let config::Config { server: serverc } =
        config::Config::from_path("config/server.toml").unwrap();
    let mut keyarr: [(u32, [u8; 16]); 12] = [(0, [0; 16]); 12];
    for (idx, (keyidx, key)) in keyarr.iter_mut().enumerate() {
        *keyidx = idx as u32 + 1;
        key.copy_from_slice(&serverc.keys[idx].as_bytes()[..16]);
    }
    let mut server = game_server::GameServer::new(
        serverc.address.parse().expect("invalid server ip address"),
        serverc.port,
        keyarr,
    )
    .unwrap();
    server.run();
    //unsafe { enet_sys::enet_deinitialize() };
}

fn setup_logger() -> Result<(), fern::InitError> {
    use fern::colors::{Color, ColoredLevelConfig};
    let colors_line = ColoredLevelConfig::new()
        .error(Color::BrightRed)
        .warn(Color::BrightYellow)
        .info(Color::White)
        .debug(Color::White)
        .trace(Color::BrightBlack);
    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{color_line}[{date}][{target}/{thread}][{level}{color_line}] {message}\x1B[0m",
                color_line = format_args!(
                    "\x1B[{}m",
                    colors_line.get_color(&record.level()).to_fg_str()
                ),
                date = chrono::Local::now().format("%H:%M:%S"),
                target = record.target(),
                thread = std::thread::current().name().unwrap_or_default(),
                level = record.level(),
                message = message,
            ));
        })
        .level(log::LevelFilter::Trace)
        .chain(std::io::stdout())
        .apply()
        .unwrap();
    Ok(())
}

use rblitz::{config, game_server};

struct EnetInit;

impl EnetInit {
    fn new() -> Self {
        unsafe { enet_sys::enet_initialize() };
        EnetInit
    }
}

impl Drop for EnetInit {
    fn drop(&mut self) {
        unsafe { enet_sys::enet_deinitialize() };
    }
}

fn main() {
    let _init = EnetInit::new();
    setup_logger().unwrap();
    let config::Config { server: serverc } =
        config::Config::from_path("config/server.toml").unwrap();
    let pconfig = config::PlayerConfig::from_path("config/players.ron").unwrap();
    let mut server = game_server::GameServer::new(
        serverc.address.parse().expect("invalid server ip address"),
        serverc.port,
        pconfig,
    )
    .unwrap();
    server.run();
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
        // replace with Trace to see all received and sent packets in the log
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .apply()
        .unwrap();
    Ok(())
}

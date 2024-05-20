use anyhow::Result;

#[derive(Debug)]
pub struct Config {
    pub(crate) port: u16,
}

impl Config {

}

pub fn load() -> Result<Config> {
    let mut args = std::env::args();
    let mut config = Config { port: 6379 };

    if args.len() == 0 {
        return Ok(config);
    }

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--port" => {
               let port = args.next().map(|p| p.parse::<u16>().ok()).flatten();


               if port.is_none() {
                   return Err(anyhow::anyhow!("Port is required"));
               }

               config.port = port.unwrap();
            }
            _ => {}
            
        }
    }

    Ok(config)
}

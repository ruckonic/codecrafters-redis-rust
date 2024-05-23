use anyhow::Result;

#[derive(Debug)]
pub enum Role {
    Master,
    Slave,
}

impl ToString for Role {
    fn to_string(&self) -> String {
        match self {
            Role::Master => "master".to_string(),
            Role::Slave => "slave".to_string(),
        }
    }
    
}

#[derive(Debug)]
pub struct Replication {
    pub(crate) host: String,
    pub(crate) port: u16,
}


#[derive(Debug)]
pub struct Config {
    pub(crate) port: u16,
    pub(crate) role: Role,
    pub(crate) replication: Option<Replication>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            port: 6379,
            role: Role::Master,
            replication: None,
        }
    }
}

pub fn load() -> Result<Config> {
    let mut args = std::env::args().peekable();
    let mut config = Config::default();


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
            "--replicaof" => {
                let replicaof = args.peek();
                config.role = Role::Slave;


                 match replicaof.clone() {
                    Some(replicaof) => {
                        let replicaof = replicaof.to_string();


                        if replicaof.starts_with("--") {
                            args.next(); 
                        } else {
                            let (host, port) = replicaof.split_once(" ").expect("Invalid replicaof parameter");

                            config.replication = Some(Replication {
                                host: host.to_string(),
                                port: port.parse::<u16>().expect("Invalid port"),
                            });

                        }
                    },
                    None => {}
                };
            }
            _ => {}
            
        }
    }

    Ok(config)
}

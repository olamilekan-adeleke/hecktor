pub struct Config {
    pub options: ConfigOptions,
    pub path: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 2 {
            return Err("⚠️  Usage: hecktor [options] [path]");
        }

        let option = args[1].clone();
        let path: String = args[2].clone();
        Ok(Config {
            options: Config::prase_str_to_options(option),
            path,
        })
    }

    fn prase_str_to_options(option: String) -> ConfigOptions {
        match option.as_str() {
            "--apk" => ConfigOptions::BuildApk,
            "--appbundle" => ConfigOptions::BuildAppBundle,
            _ => ConfigOptions::BuildApk,
        }
    }
}

pub enum ConfigOptions {
    BuildApk,
    BuildAppBundle,
}

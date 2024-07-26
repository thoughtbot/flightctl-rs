use super::{Application, Config, Release};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Selector {
    #[structopt(short, long)]
    pub application: Option<String>,

    #[structopt(short, long)]
    pub environment: Option<String>,
}

impl Selector {
    pub fn apply(self, config: &Config) -> anyhow::Result<&Release> {
        let application = self.application(&config)?;

        match &self.environment {
            Some(name) => {
                for release in &config.releases {
                    if &release.environment == name && release.application == application.name {
                        return Ok(release);
                    }
                }
                Err(anyhow::Error::msg(
                    "No release found for that application and environment",
                ))
            }
            None => {
                if config.releases.len() == 1 {
                    Ok(&config.releases[0])
                } else {
                    Err(anyhow::Error::msg(format!(
                        "More than one release for application {}",
                        application.name,
                    )))
                }
            }
        }
    }

    pub fn merge(&self, other: &Selector) -> Selector {
        Selector {
            application: other
                .application
                .clone()
                .or_else(|| self.application.clone()),
            environment: other
                .environment
                .clone()
                .or_else(|| self.environment.clone()),
        }
    }

    fn application<'a>(&self, config: &'a Config) -> anyhow::Result<&'a Application> {
        match &self.application {
            Some(name) => {
                for application in &config.applications {
                    if &application.name == name {
                        return Ok(application);
                    }
                }
                Err(anyhow::Error::msg("No application found by that name"))
            }
            None => {
                match config.applications.len() {
                    0 => Err(anyhow::Error::msg("No applications found in config")),
                    1 => Ok(&config.applications[0]),
                    _ => Err(anyhow::Error::msg("More than one application")),
                }
            }
        }
    }
}

use crate::models::rune;
use anyhow::Context;
use lazy_regex::regex_captures;
use reqwest::Method;
use url::Url;

#[derive(Debug)]
pub struct Connection {
    pub url: Url,
    pub auth: String,
    pub client: reqwest::Client,
}

impl Connection {
    pub fn new() -> anyhow::Result<Self> {
        let (url, auth) = Self::get_url_and_auth()?;

        Ok(Connection {
            url,
            auth,
            client: reqwest::Client::builder()
                .danger_accept_invalid_certs(true)
                .build()?,
        })
    }

    fn get_url_and_auth() -> anyhow::Result<(Url, String)> {
        let system =
            sysinfo::System::new_with_specifics(sysinfo::RefreshKind::new().with_processes(
                sysinfo::ProcessRefreshKind::new().with_cmd(sysinfo::UpdateKind::Always),
            ));

        system
            .processes()
            .values()
            .find_map(|p| {
                macro_rules! extract_arg {
                    ($regex:literal) => {
                        p.cmd()
                            .into_iter()
                            .find_map(|arg| Some(regex_captures!($regex, arg.to_str()?)?.1))?
                    };
                }

                if !p.cmd().get(0)?.to_str()?.contains("LeagueClientUx.exe") {
                    return None;
                }

                let auth = extract_arg!(r"^--remoting-auth-token=([\w-]*)$").to_owned();
                let port = extract_arg!(r"^--app-port=(\d*)$").parse().unwrap();

                let mut url = Url::parse("https://127.0.0.1").unwrap();
                url.set_port(Some(port)).unwrap();

                Some((url, auth))
            })
            .context("Could not find LCU process")
    }

    pub async fn rune_pages(&self) -> anyhow::Result<Vec<rune::Page>> {
        Ok(self
            .request(Method::GET, "lol-perks/v1/pages")
            .send()
            .await?
            .json::<Vec<rune::Page>>()
            .await?)
    }

    fn request(&self, method: Method, path: &str) -> reqwest::RequestBuilder {
        let mut url = self.url.clone();
        url.set_path(path);

        self.client
            .request(method, url)
            .basic_auth("riot", Some(&self.auth))
    }
}

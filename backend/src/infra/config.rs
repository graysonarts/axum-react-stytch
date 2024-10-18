use envconfig::Envconfig;

#[derive(Debug, Envconfig)]
pub struct Config {
    pub stytch_workspace_id: String,
    pub stytch_project_id: String,
    pub stytch_secret: String,
    pub frontend_url: String,
}

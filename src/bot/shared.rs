use super::api::Api;

pub struct Shared {
    pub api: Api,
}

impl Shared {
    pub fn new(api: Api) -> Self {
        Self { api }
    }
}

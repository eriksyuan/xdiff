use anyhow::{Result, Ok};
use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use tokio::fs;


use crate::{ExtraArgs, req::RequestProfile};
#[derive(Debug, Serialize, Deserialize)]
pub struct DiffConfig {
    #[serde(flatten)]
    pub profiles: HashMap<String, DiffProfile>,
}

impl DiffConfig {
    pub fn from_yaml(content: &str) -> Result<Self> {
        Ok(serde_yaml::from_str(content)?)
    }
    pub async fn load_yaml(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path).await?;
        Self::from_yaml(&content)
    }
    pub fn get_profile(&self,name:&String) -> Option<&DiffProfile>{
        self.profiles.get(name) 
    }
}



#[derive(Debug, Serialize, Deserialize)]
pub struct DiffProfile {
    pub req1: RequestProfile,
    pub req2: RequestProfile,
    pub res: ResponseProfile,
}


impl DiffProfile {
    pub async fn diff(&self,_args:&ExtraArgs) -> Result<()>{

        todo!()
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseProfile {
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub skip_headers: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub skip_body: Vec<String>,
}

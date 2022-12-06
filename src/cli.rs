use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};

use crate::ExtraArgs;

#[derive(Parser, Clone, Debug)]
#[clap(version, author, about, long_about=None)]
pub struct Args {
    #[command(subcommand)]
    pub action: Action,
}

#[derive(Subcommand, Clone, Debug)]
#[non_exhaustive]
pub enum Action {
    /// 比较两个请求响应体的不同
    Run(RunArgs),
}

#[derive(Parser, Clone, Debug)]
pub struct RunArgs {
    /// profile name
    #[clap(short, long, value_parser)]
    pub profile: String,
    /// Overrides args query，header，body。
    /// for query use `-e key=value`
    /// for header use `-e %key=value`
    /// for body use `-e @key=value`
    #[clap(short, long, value_parser=value_parser_keyval,num_args=0..6)]
    pub extra_params: Vec<KeyVal>,

    /// config file
    #[clap(short, long, value_parser)]
    pub config: Option<String>,
}

#[derive(Clone, Debug)]
pub enum KeyValType {
    Header,
    Query,
    Body,
}

#[derive(Clone, Debug)]
pub struct KeyVal {
    key: String,
    value: String,
    key_type: KeyValType,
}

fn value_parser_keyval(s: &str) -> Result<KeyVal> {

    let mut split = s.splitn(2, '=');
    let key = split.next().ok_or_else(||anyhow!("invalid key value"))?.trim();
    let value = split.next().ok_or_else(||anyhow!("invalid key value"))?.trim().to_string();

    let (key, key_type) = match s.chars().nth(0) {
        Some('%') => (key[1..].to_string(), KeyValType::Header),
        Some('@') => (key[1..].to_string(), KeyValType::Body),
        _ => (s.to_string(), KeyValType::Body),
    };

    Ok(KeyVal {
        key,
        value,
        key_type,
    })
}

impl From<Vec<KeyVal>> for ExtraArgs {
    fn from(e: Vec<KeyVal>) -> Self {
        let mut headers = Vec::new();
        let mut body = Vec::new();
        let mut query = Vec::new();

        for args in e {
            match args.key_type {
                KeyValType::Body => body.push((args.key, args.value)),
                KeyValType::Header => headers.push((args.key, args.value)),
                _ => query.push((args.key, args.value)),
            }
        }
        Self {
            headers,
            body,
            query,
        }
    }
}

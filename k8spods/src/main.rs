#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate dirs;

pub mod config;

use std::path::{Path, PathBuf};
use std::env;
use std::process::Command;
use serde_json::{Value};

fn main() {
    let config_path= get_config_path();
    let config_path = config_path.to_str().unwrap();
    let c = config::Config::from_file(config_path.to_string())
        .expect("Failed to load config");

    println!("Kubernetes");
    println!("---");
    for ns in &c.namespaces {
        println!("{}", ns);

        let pod_json = run_list_pods(&c, ns.as_str()).expect("Unable to list pods");

        let data: Value = serde_json::from_str(pod_json.as_str()).ok().expect("Error parsing json");

        let items = &data["items"];

        for pods in items.as_array() {
            for pod in pods {
                let name = &pod["metadata"]["name"].as_str().unwrap();
                let phase = &pod["status"]["phase"].as_str().unwrap();
                let image = &pod["spec"]["containers"][0]["image"].as_str().unwrap();
                let parts = image.rsplitn(2, ":").collect::<Vec<&str>>();
                let tag = parts[0];

                if *phase == "Running" || *phase == "Succeeded" {
                    println!("--:white_check_mark: {} ({}): {} | color=#00000", name, tag, phase);
                } else {
                    println!("--:large_orange_diamond: {} ({}): {} | color=#ff0000", name, tag, phase);
                }
            }
        }
    }

}


fn get_k8s_token(c: &config::Config) -> Option<String> {
    let json = Command::new(&c.iam_authenticator_path)
        .args(&[
            "token",
            "-i",
            c.cluster_name.as_str(),
        ])
        .output().ok()
        .and_then(|r| Some(r.stdout.iter().map(|&c| c as char).collect::<String>()))?;


    let data: Value = serde_json::from_str(json.as_str()).ok()?;

    Some(data["status"]["token"].as_str().unwrap().to_string())
}

fn run_list_pods(c: &config::Config, ns: &str) -> Option<String> {
    let token = get_k8s_token(&c)?;
    Command::new(&c.kubectl_path)
	.args(&["\
        get",
        "pods",
        format!("--token={}", &token).as_str(),
        format!("--namespace={}", ns).as_str(),
        "-ojson"
    ])
	.output().ok()
	.and_then(|r| Some(r.stdout.iter().map(|&c| c as char).collect::<String>()))
}

fn get_config_path() -> PathBuf {
    let config_path = std::env::var("CONFIG_PATH");

    match config_path {
        Ok(path) => {
            let p = Path::new(path.as_str());
            if p.is_absolute() {
                p.to_path_buf()
            } else {
                env::current_dir().unwrap().join(p)
            }
        },
        _ => {
            let home = dirs::home_dir().unwrap();
            home.join(".config/bitbar/k8spods.toml")
        }
    }
}


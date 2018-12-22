extern crate serde_json;

use std::process::Command;
use serde_json::{Value};

fn run_list_pods(ns: &str) -> Option<String> {
    Command::new("kubectl")
	.args(&["get", "pods", format!("-n{}", ns).as_str(), "-ojson"])
	.output().ok()
	.and_then(|r| Some(r.stdout.iter().map(|&c| c as char).collect::<String>()))
}

fn main() {
    let env_namespaces: String = std::env::var("NAMESPACES")
        .expect("NAMESPACES not defined");

    let namespaces = env_namespaces.split(" ").collect::<Vec<&str>>();

    println!("Kubernetes");
    println!("---");
    for ns in namespaces {
        println!("{}", ns);
        let pod_json = run_list_pods(ns).expect("Error");
        let data: Value = serde_json::from_str(pod_json.as_str()).ok().expect("Error");

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

extern crate serde_json;

use std::process::Command;
use serde_json::{Value};

fn run_list_pods() -> Option<String> {
    Command::new("kubectl")
	.args(&["get", "pods", "-nstaging", "-ojson"])
	.output().ok()
	.and_then(|r| Some(r.stdout.iter().map(|&c| c as char).collect::<String>()))
}

fn main() {
    let pod_json = run_list_pods().expect("Error");
    let data: Value = serde_json::from_str(pod_json.as_str()).ok().expect("Error");

    let items = &data["items"];
    for item in items.as_array() {
	for pod_spec in item {
	    println!("{:?}", pod_spec["spec"]["containers"][0]["image"]);

    }
}

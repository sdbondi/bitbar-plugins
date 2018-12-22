extern crate rusoto_core;
extern crate rusoto_codebuild;

use rusoto_core::Region;
use rusoto_codebuild::{
  CodeBuild,
  CodeBuildClient,
  ListBuildsForProjectInput,
  BatchGetBuildsInput,
  Build,
};

const MAX_BUILDS: usize = 5;

fn get_builds_for_project(client: &CodeBuild, name: String, max: usize) -> Option<Vec<Build>> {
    let build_result = client.list_builds_for_project(ListBuildsForProjectInput{
      project_name: name,
      sort_order: None,
      next_token: None,
    }).sync();

    let builds = match build_result {
      Ok(b) => b,
      Err(e) => panic!(e),
    };

    let builds = match builds.ids {
      Some(ids) => client.batch_get_builds(BatchGetBuildsInput{ ids }).sync(),
      _ => return None,
    };

    match builds {
	Ok(b) => b.builds.and_then(|builds| {
	    Some(builds[..max].to_vec())
	}),
	_ => None,
    }
}

fn main() {
  println!("CodeBuild");
  println!("---");

  let projects = vec!["bigneon-api", "bigneon-web"];

  let client = CodeBuildClient::new(Region::UsWest2);

  for p in projects {
      let builds = get_builds_for_project(&client, p.to_string(), MAX_BUILDS);
      println!("{}", p);

      if let Some(builds) = builds {
	  for b in builds {
	      println!("--{} - {}", b.source_version.unwrap_or("null".to_string()), b.build_status.unwrap_or("null".to_string()));
	  }
      }
  }

}

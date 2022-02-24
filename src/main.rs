use std::env;
use gitlab_api::{Pilot, VCS};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let pilot = Pilot::new(&args[1], VCS::GitLab);
    let reviews = pilot.requests().await;

    println!("{:?}", reviews);

    for m in reviews {
        let notes = pilot.comments(&m).await;

        println!("{:?}", notes);

        // for n in notes {
        //     let file = client.get_raw_file(m.project_id, n.position.unwrap()).await;
        //     println!("{:?}", file)
        // }
    }
}

use post_manager::PostManager;

pub mod post_manager;

fn main() {
    let mut post_manager = PostManager {
        posts: vec![]
    };
    post_manager.start();
}

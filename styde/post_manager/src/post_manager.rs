mod post;

use post::{Post, Priority};
use std::fs::File;
use std::io::stdin;
use std::io::Write;
use std::path::Path;

fn read_input (input_message: &str, error_message: &str) -> String {
    let msg = format!("{}", input_message);

    print!("{}", msg);
    std::io::stdout().flush().unwrap();
    let mut input_value = "".to_owned();
    stdin().read_line(&mut input_value).expect(error_message);

    return input_value.trim().to_owned();
}

pub struct PostManager {
    pub posts: Vec<Post>,
}

impl PostManager {

    pub fn start (&mut self) {
        loop {
            let _ = self.listen_command();
        }
    }

    fn listen_command (&mut self) {
        let commands = ["show posts", "add post", "update post", "delete post"];

        println!("\nCommands:");
        for (i, comm) in commands.iter().enumerate() {
            println!("{}: {}", i + 1, comm);
        }

        let command_input = read_input("\nInput index of commands: ", "Error input user message");

        match command_input.as_str().trim() {
            "1" => self.print_posts(),
            "2" => self.add_post(),
            "3" => self.update_post(),
            "4" => self.delete_post(),
            _ => println!("This command not found")
        };
    }

    fn add_post (&mut self) {
        let title = read_input("Input title:", "Error input user message");
        let text = read_input("Input text:\n", "Error input user message");
        let priority_message = read_input("\nInput priority index or name: \n 1. low\n 2. medium\n 3. high\nInput: ", "Error input user message");

        let priority = match priority_message.as_str().trim() {
            "1" => Priority::Low,
            "2" => Priority::Medium,
            "3" => Priority::High,
            "low" => Priority::Low,
            "medium" => Priority::Medium,
            "high" => Priority::High,
            _ => Priority::Medium
        };

        let post = Post::new(text, title, priority);

        let _ = &self.posts.push(post);
    }

    fn delete_post (&mut self) {
      println!("Посты: ");
      for (i, title) in self.posts.iter().enumerate() {
        println!(" {}: {}", i + 1, title.title);
      }

       let title = read_input("\nInput title of post: ", "Error input user message");

       match self.find_post(title.to_owned()) {
        Some(index) => {
          self.posts.remove(index);
          println!("Remove posts by title {} successfully", title);
        },
        None => println!("Remove posts by title {} failed", title)
       }
    }

    fn update_post (&mut self) {
      println!("Посты: ");
      for (i, title) in self.posts.iter().enumerate() {
        print!(" {}: {}", i + 1, title.title);
      }

       let title_search = read_input("\nInput title of post: ", "Error input user message");

        let title = read_input("Input title or ignore:", "Error input user message");
        let text = read_input("Input text or ignore:\n", "Error input user message");
        let priority_message = read_input("\nInput priority index or name or ignore: \n 1. low\n 2. medium\n 3. high\nInput: ", "Error input user message");

       match self.find_post(title_search.to_owned()) {
        Some(index) => {
          match self.posts.get_mut(index) {
            Some(post) => {
              if title.to_owned().trim().len() > 0 {
                post.title = title.clone();
              }
              if text.to_owned().trim().len() > 0 {
                post.text = text;
              }
              if priority_message.to_owned().trim().len() > 0 {
                let priority = match priority_message.as_str().trim() {
                    "1" => Priority::Low,
                    "2" => Priority::Medium,
                    "3" => Priority::High,
                    "low" => Priority::Low,
                    "medium" => Priority::Medium,
                    "high" => Priority::High,
                    _ => Priority::Medium
                };
                post.priority = priority;
              }
            println!("Update posts by title {} successfully", title);
            }
            None => println!("Update posts by title {} failed", title)
          }
        },
        None => println!("Update posts by title {} failed", title)
       }
    }

    fn print_posts (&self) {
        println!("");
        for post in self.posts.iter() {
            post.print_post();
        }
    }

    fn find_post (&self, title: String) -> Option<usize> {
        self.posts.iter().position(|post | post.title == title)
    }


}

pub enum Priority 
{
    Low,
    Medium,
    High,
}

pub struct Post {
    pub title: String,
    pub text: String,
    pub priority: Priority
}

impl Post {
    pub fn new (text: String, title: String, priority: Priority) -> Post {
        Self {
            text,
            title,
            priority,
        }
    }

    pub fn print_post (&self) {
        let priority = match self.priority {
            Priority::Low => "low",
            Priority::Medium => "medium",
            Priority::High => "high"
        };

        print!("{} | ", self.title);
        print!("{}\n", priority);
        print!("{}\n", self.text);
    }
}
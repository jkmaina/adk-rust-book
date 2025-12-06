use std::sync::Arc;
use std::thread;

// Minimal stand-in for a model type
#[derive(Debug)]
struct GeminiModel {
    api_key: String,
    name: String,
}

impl GeminiModel {
    fn new(api_key: &str, name: &str) -> Self {
        Self { api_key: api_key.to_string(), name: name.to_string() }
    }

    // A simple method that borrows &self (no mutation), safe to call from multiple threads
    fn infer(&self, input: &str) -> String {
        format!("{} -> [{}] (model={})", self.name, input, self.api_key)
    }
}

fn main() {
    // Pretend we loaded a model with an API key
    let api_key = "sk-REDACTED";
    let model = GeminiModel::new(api_key, "gemini-2.0-flash-exp");

    // Wrap in Arc to share ownership between threads
    let model = Arc::new(model);

    println!("Main: Arc strong count after create = {}", Arc::strong_count(&model));

    // Spawn some worker threads that share the same model instance
    let mut handles = Vec::new();
    for i in 1..=3 {
        let model_clone = Arc::clone(&model);
        let handle = thread::spawn(move || {
            // Each thread uses the shared model without taking ownership
            let input = format!("message {}", i);
            let out = model_clone.infer(&input);
            println!("Thread {} output: {}", i, out);
            // show the count inside thread (clone holds +1 while thread runs)
            println!("Thread {}: Arc strong count = {}", i, Arc::strong_count(&model_clone));
        });
        handles.push(handle);
        println!("Main: spawned thread {}, Arc strong count = {}", i, Arc::strong_count(&model));
    }

    // Wait for all threads to finish
    for h in handles {
        h.join().expect("thread panicked");
    }

    println!("Main: Arc strong count after join = {}", Arc::strong_count(&model));
    println!("All done — the same `GeminiModel` instance was shared safely across threads using `Arc`.");
}

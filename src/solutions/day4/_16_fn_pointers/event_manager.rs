struct EventManager {
    on_start: Vec<Box<dyn FnMut() -> ()>>,
    on_message: Vec<Box<dyn Fn(String) -> String>>,
    on_shutdown: Vec<Box<dyn FnOnce() -> bool>>,
}

impl EventManager {
    fn new() -> Self {
        EventManager {
            on_start: Vec::new(),
            on_message: Vec::new(),
            on_shutdown: Vec::new(),
        }
    }

    fn register_start_handler(&mut self, handler: impl FnMut() -> () + 'static) {
        self.on_start.push(Box::new(handler));
    }

    fn register_message_handler(&mut self, handler: impl Fn(String) -> String + 'static) {
        self.on_message.push(Box::new(handler));
    }

    fn register_shutdown_handler(&mut self, handler: impl FnOnce() -> bool + 'static) {
        self.on_shutdown.push(Box::new(handler));
    }

    fn trigger_start(&mut self) {
        for handler in &mut self.on_start {
            handler();
        }
    }

    fn process_message(&self, message: String) -> Vec<String> {
        let mut vector = Vec::new();

        for handler in &self.on_message {
            let result = handler(message.to_string());
            vector.push(result);
        }
        vector
    }

    fn shutdown(self) -> bool {
        let mut all_success: bool = true;
        for handler in self.on_shutdown {
            if !handler() {
                all_success = false;
            }
        }
        all_success
    }
}

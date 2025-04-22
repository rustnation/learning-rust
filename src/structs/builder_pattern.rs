#[derive(Debug)]
struct Computer {
    cpu: String,
    memory: u32,
    storage: u32,
}

impl Computer {
    fn new(cpu: String, memory: u32, storage: u32) -> Self {
        Self {
            cpu,
            memory,
            storage,
        }
    }
}

impl Computer {
    fn upgrade_cpu(&mut self, new_cpu: String) -> &mut Self {
        self.cpu = new_cpu;
        self
    }

    fn upgrade_memory(&mut self, new_memory: u32) -> &mut Self {
        self.memory = new_memory;
        self
    }

    fn upgrade_storage(&mut self, new_storage: u32) -> &mut Self {
        self.storage = new_storage;
        self
    }
}

pub fn index(show: bool) {
    if show {
        let mut computer = Computer::new("M3 Max".to_string(), 64, 2048);

        // Builder pattern style
        computer
            .upgrade_cpu("M4 Max".to_string())
            .upgrade_memory(128)
            .upgrade_storage(4096);

        println!("{:?}", computer);
    }
}

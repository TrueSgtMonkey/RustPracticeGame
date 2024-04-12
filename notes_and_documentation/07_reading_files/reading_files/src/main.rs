enum ReadMode {
    Keyboard,
    Mouse,
    Error,
}

struct ActionMap {
    pub key_map: [u16;4],
    pub mouse_map: [u16;2],
}

fn main() {
    let mut action_map = ActionMap::new();
    action_map.parse_contents("./config/input_config.cfg");
}

impl ActionMap {
    pub fn new() -> Self {
        Self {
            key_map: [0, 1, 2, 3],
            mouse_map: [0, 1],
        }
    }

    pub fn parse_contents(&self, filename: &str) {
        let mut read_mode: ReadMode = ReadMode::Keyboard;

        for line in std::fs::read_to_string(filename).unwrap().lines() {
            if line.chars().count() == 0 {
                continue;
            }

            if line.as_bytes()[0] == '[' as u8 {
                read_mode = self.get_read_mode(line);
            }

            match read_mode {
                
            }
        }
    }

    fn get_read_mode(&self, line: &str) -> ReadMode {
        match line {
            "[Keyboard]" => ReadMode::Keyboard,
            "[Mouse]" => ReadMode::Mouse,
            _ => ReadMode::Error,
        }
    }
}

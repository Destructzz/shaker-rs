use serde::{Deserialize, Serialize};
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Default for Position {
    fn default() -> Self {
        Position { x: 0, y: 0 }
    }
}

pub struct Config;

impl Config {
    pub fn init() -> Result<Position, Box<dyn std::error::Error>> {
        let config_path = Self::get_config_path()?;
        
        // Если файл не существует или пустой - создаём дефолтный
        if !config_path.exists() || fs::metadata(&config_path)?.len() == 0 {
            let default_pos = Position::default();
            Self::save(&config_path, &default_pos)?;
            return Ok(default_pos);
        }
        
        // Читаем существующий файл
        let mut file = File::open(&config_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        
        let position: Position = serde_json::from_str(&contents)?;
        Ok(position)
    }
    
    pub fn save_position(position: &Position) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = Self::get_config_path()?;
        Self::save(&config_path, position)
    }
    
    fn get_config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let mut exe_path = std::env::current_exe()?;
        exe_path.pop(); // Убираем имя exe, оставляем директорию
        exe_path.push("config.json");
        Ok(exe_path)
    }
    
    fn save(path: &PathBuf, position: &Position) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(position)?;
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }
}

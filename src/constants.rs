use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

use sqlx::migrate::Migrator;

use crate::data::settings::SettingsInfo;


// App
pub const APP_NAME: &str = "Shelf";
pub const APP_VERSION: &str = "0.2.0";


// Data
pub const DATA_DIR_NAME: &str = ".shelf";
pub const DB_FILE_NAME: &str = "shelf.db";
pub const SETTINGS_FILE_NAME: &str = "settings.json";

// Looks for migrations in the "./migrations" directory at compile time
pub static MIGRATOR: Migrator = sqlx::migrate!();

// Intialized once
pub static DATA_DIR_PATH: OnceLock<PathBuf> = OnceLock::new();

// Cached settings
pub static SETTINGS: OnceLock<Mutex<SettingsInfo>> = OnceLock::new();


// Groups
// Special groups except default don't exists in the database and are created dynamically
pub const DEFAULT_GROUP_ID: i32 = 1;
pub const RECENT_GROUP_ID: i32 = -1;
pub const FAVOURITES_GROUP_ID: i32 = -2;

// Max items shown in recent group
pub const RECENT_ITEMS_LIMIT: i32 = 50;

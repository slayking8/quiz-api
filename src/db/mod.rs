pub mod seeder;
pub mod setup;

// Re-export the functions so main.rs can call db::setup_database() directly
pub use seeder::seed_database;
pub use setup::setup_database;

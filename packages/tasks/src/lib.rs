mod tasks;
pub use tasks::factorial::factorial as factorial_task;
pub use tasks::fibonacci::fibonacci as fibonacci_task;
pub use tasks::guessing_game::guessing_game as guessing_game_task;
pub use tasks::l4and5struct::l4and5struct as l4and5struct_task;
pub use tasks::l6collection::l6collection as l6collection_task;
pub use tasks::palindrome::palindrome as palindrome_task;

// #[path = "../bin/guessing_game.rs"]
// mod guessing_game_bin;
// pub use guessing_game_bin::guessing_game as guessing_game_task;

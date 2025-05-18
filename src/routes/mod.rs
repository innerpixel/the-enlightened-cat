//! # Routes Module
//! 
//! This module organizes all HTTP route handlers for The Enlightened Cat application.
//! In Rust, `mod.rs` files are used to define and organize submodules within a directory.
//! 
//! ## Structure
//! 
//! - `chat`: Handles API endpoints for chat interactions with the Enlightened Cat
//! - `pages`: Handles HTML page rendering for the website frontend
//! - `wisdom`: Handles API endpoints for daily wisdom quotes
//!
//! Each of these is a separate module (Rust file) with its own functionality.
//! The `pub` keyword makes these modules publicly accessible from outside this module.

pub mod chat;    // Makes the chat.rs module public and available
pub mod pages;   // Makes the pages.rs module public and available
pub mod wisdom;  // Makes the wisdom.rs module public and available

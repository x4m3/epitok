//! # Introduction
//!
//! This library is designed to remove the "tokens" in the Epitech school.
//!
//! A token is a piece of paper given to students with a number on it to confirm their presence to an event.
//! Then the students needs to enter their token on the intranet to confirm their presence.
//! But often this piece of paper can be forgotten easily, and for every event new tokens have to be printed.
//!
//! The goal is to simplify students' and school's staff lives by not having to deal these tokens anymore.
//!
//! ## Warning
//!
//! This library will be useful only to people who are in possession of a privileged Epitech account (astek, aer, adm, pedago).
//! So if you are just an Epitech student, this library won't be helpful for you at all.

// internal modules
mod intra;
mod student;

// public modules
pub mod auth;
pub mod event;

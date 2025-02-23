#![cfg_attr(rustfmt, rustfmt_skip)]
// This file is autogenerated by build.rs, do not edit.

mod color_command;
mod get_linked_account_command;
mod uptime_command;
pub mod tags;
pub mod utils;

pub fn get_all_commands() -> Vec<poise::Command<crate::Data, crate::types::Error>> {
    vec![color_command::color(), get_linked_account_command::get_linked_account(), tags::dtag_command::dtag(), tags::tag_command::tag(), uptime_command::uptime()]
}
use common::glob_export;

pub mod cache;
pub mod login;

glob_export!(add_player);
glob_export!(add_painting);
glob_export!(animate);
glob_export!(available_actor_identifiers);
glob_export!(available_commands);
glob_export!(biome_definition_list);
glob_export!(block_event);
glob_export!(block_pick_request);
glob_export!(book_edit);
glob_export!(boss_event);
glob_export!(camera_shake);
glob_export!(change_dimension);
glob_export!(client_bound_debug_renderer);
glob_export!(command_output);
glob_export!(command_request);
glob_export!(connect_automation_client);
glob_export!(container_close);
glob_export!(container_open);
glob_export!(death_info);
glob_export!(event);
glob_export!(game_rules_changed);
glob_export!(generic_level_event);
glob_export!(interact);
glob_export!(level_chunk);
glob_export!(level_event);
glob_export!(mob_effect);
glob_export!(move_player);
glob_export!(network_chunk_publisher_update);
glob_export!(packet);
glob_export!(play_sound);
glob_export!(play_status);
glob_export!(player_list);
glob_export!(request_ability);
glob_export!(respawn);
glob_export!(set_commands_enabled);
glob_export!(set_default_game_mode);
glob_export!(set_difficulty);
glob_export!(set_local_player_as_initialized);
glob_export!(set_player_gamemode);
glob_export!(set_scoreboard_identity);
glob_export!(set_time);
glob_export!(set_title);
glob_export!(show_credits);
glob_export!(show_profile);
glob_export!(simple_event);
glob_export!(spawn_experience_orb);
glob_export!(text);
glob_export!(tick_sync);
glob_export!(toast_request);
glob_export!(traits);
glob_export!(transfer);
glob_export!(update_dynamic_enum);
glob_export!(update_fog_stack);
glob_export!(update_skin);
glob_export!(violation_warning);

/// ID of Minecraft game packets.
pub const GAME_PACKET_ID: u8 = 0xfe;
/// Semver version that this server supports.
pub const CLIENT_VERSION_STRING: &str = "1.19.60";
/// Protocol version that this server supports.
pub const NETWORK_VERSION: u32 = 567;

const Commands = {
	// Game Instances
	UpsertGameInstance: 'upsert_game_instance',
	FetchTheGameInstance: 'fetch_the_game_instance',
	UpdateGameInstance: 'update_game_instance',
	ResetGameInstance: 'reset_game_instance',
	// Mod Info
	CreateModInfo: 'create_mod_info',
	FetchAllMods: 'fetch_all_mods',
	UpdateModInfo: 'update_mod_info',
	DeleteModInfo: 'delete_mod_info',
	// Profile
	CreateProfile: 'create_profile',
	FetchAllProfiles: 'fetch_all_profiles',
	UpdateProfile: 'update_profile',
	DeleteProfile: 'delete_profile',
	EnableMod: 'enable_mod',
	DisableMod: 'disable_mod',
	GetActiveMods: 'get_active_mods',
	LaunchGame: 'launch_game'
};

export default Commands;
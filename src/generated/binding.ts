 // This file has been generated by Specta. DO NOT EDIT.

export type GameInstance = { id: string; path: string; mod_engine_path: string | null }

export type GameInstanceDataModel = { path: string | null; mod_engine_path: string | null }

export type ModInfo = { id: string; deployment_path: string | null; is_dll: boolean; name: string; path: string }

export type ModInfoDataModel = { deployment_path: string | null; is_dll: boolean; name: string; path: string }

export type Profile = { id: string; name: string }

export type ProfileDataModel = { name: string }

export type TauRpcGameInstanceApiInputTypes = { proc_name: "upsert"; input_type: { __taurpc_type: GameInstanceDataModel } } | { proc_name: "fetch"; input_type: null } | { proc_name: "update"; input_type: { __taurpc_type: GameInstance } } | { proc_name: "reset"; input_type: null }

export type TauRpcGameInstanceApiOutputTypes = { proc_name: "upsert"; output_type: GameInstance } | { proc_name: "fetch"; output_type: GameInstance } | { proc_name: "update"; output_type: GameInstance } | { proc_name: "reset"; output_type: GameInstance }

export type TauRpcModInfoApiInputTypes = { proc_name: "create"; input_type: { __taurpc_type: ModInfoDataModel } } | { proc_name: "update"; input_type: { __taurpc_type: ModInfo } } | { proc_name: "delete"; input_type: { __taurpc_type: ModInfo } } | { proc_name: "fetch_all"; input_type: null }

export type TauRpcModInfoApiOutputTypes = { proc_name: "create"; output_type: ModInfo } | { proc_name: "update"; output_type: ModInfo | null } | { proc_name: "delete"; output_type: ModInfo | null } | { proc_name: "fetch_all"; output_type: ModInfo[] }

export type TauRpcModManagementApiInputTypes = { proc_name: "enable"; input_type: [string, string] } | { proc_name: "disable"; input_type: [string, string] } | { proc_name: "fetch_active"; input_type: { __taurpc_type: string } } | { proc_name: "launch_game"; input_type: { __taurpc_type: Profile } }

export type TauRpcModManagementApiOutputTypes = { proc_name: "enable"; output_type: null } | { proc_name: "disable"; output_type: null } | { proc_name: "fetch_active"; output_type: ModInfo[] } | { proc_name: "launch_game"; output_type: null }

export type TauRpcProfileApiInputTypes = { proc_name: "create"; input_type: { __taurpc_type: ProfileDataModel } } | { proc_name: "update"; input_type: { __taurpc_type: Profile } } | { proc_name: "delete"; input_type: { __taurpc_type: Profile } } | { proc_name: "fetch_all"; input_type: null }

export type TauRpcProfileApiOutputTypes = { proc_name: "create"; output_type: Profile } | { proc_name: "update"; output_type: Profile | null } | { proc_name: "delete"; output_type: Profile | null } | { proc_name: "fetch_all"; output_type: Profile[] }

const ARGS_MAP = {"api.mod":"{\"create\":[\"data\"],\"update\":[\"data\"],\"delete\":[\"data\"],\"fetch_all\":[]}","api.profile":"{\"create\":[\"data\"],\"fetch_all\":[],\"update\":[\"data\"],\"delete\":[\"data\"]}","api.game":"{\"fetch\":[],\"reset\":[],\"update\":[\"data\"],\"upsert\":[\"data\"]}","api.mod.manage":"{\"enable\":[\"profile_id\",\"mod_id\"],\"launch_game\":[\"profile\"],\"disable\":[\"profile_id\",\"mod_id\"],\"fetch_active\":[\"profile_id\"]}"}
import { createTauRPCProxy as createProxy } from "taurpc"

export const createTauRPCProxy = () => createProxy<Router>(ARGS_MAP)

type Router = {
	'api.mod': [TauRpcModInfoApiInputTypes, TauRpcModInfoApiOutputTypes],
	'api.profile': [TauRpcProfileApiInputTypes, TauRpcProfileApiOutputTypes],
	'api.mod.manage': [TauRpcModManagementApiInputTypes, TauRpcModManagementApiOutputTypes],
	'api.game': [TauRpcGameInstanceApiInputTypes, TauRpcGameInstanceApiOutputTypes],
}
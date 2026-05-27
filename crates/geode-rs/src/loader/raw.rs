use super::types::{
    ComparableVersionInfo, LoadProblem, MatJsonType, MatJsonValue, ModMetadata,
    ModMetadataDependency, ModMetadataIncompatibility, ModMetadataIssuesInfo, ModMetadataLinks,
    ModRequestedAction, Severity, VersionInfo,
};
use crate::geode_bind;
use crate::loader::{ByteSpan, GeodeResult};
use crate::stl::{
    StlOptional, StlPath, StlSharedPtr, StlString, StlStringView, StlVector, ZStringView,
};
use crate::tulip::{HandlerMetadata, HookMetadata};
use std::ffi::c_void;

// Note: if you want to add or change bindings here, grab the symbol you want from
// the geode dynamic library, like this:
//
//   dumpbin /exports Geode.dll
//     or for Android...
//   llvm-nm -D Geode.android{32/64}.so (or check .sym file)
//     or for Mac...
//   llvm-nm -gU Geode.dylib
//     or for iOS...
//   llvm-nm -gU Geode.ios.dylib
//
// When returning views/wrappers on MSVC, you may have to add `method_sret` to the return type.
// (see geode-macros/src/bind.rs for implementation of this macro)
//
// After adding a raw binding, implement the safe wrapper counterpart.

geode_bind! {
    pub unsafe fn loader_get() -> *mut c_void {
        win: "?get@Loader@geode@@SAPEAV12@XZ",
        mac_intel: "_ZN5geode6Loader3getEv",
        mac_arm: "_ZN5geode6Loader3getEv",
        ios: "_ZN5geode6Loader3getEv",
        android32: "_ZN5geode6Loader3getEv",
        android64: "_ZN5geode6Loader3getEv",
    }

    pub unsafe fn loader_take_next_mod(loader: *mut c_void) -> *mut c_void {
        win: "?takeNextMod@Loader@geode@@IEAAPEAVMod@2@XZ",
        mac_intel: "_ZN5geode6Loader11takeNextModEv",
        mac_arm: "_ZN5geode6Loader11takeNextModEv",
        ios: "_ZN5geode6Loader11takeNextModEv",
        android32: "_ZN5geode6Loader11takeNextModEv",
        android64: "_ZN5geode6Loader11takeNextModEv",
    }

    pub unsafe fn loader_get_loading_state(loader: *mut c_void) -> i32 {
        win: "?getLoadingState@Loader@geode@@QEAA?AW4LoadingState@12@XZ",
        mac_intel: "_ZN5geode6Loader15getLoadingStateEv",
        mac_arm: "_ZN5geode6Loader15getLoadingStateEv",
        ios: "_ZN5geode6Loader15getLoadingStateEv",
        android32: "_ZN5geode6Loader15getLoadingStateEv",
        android64: "_ZN5geode6Loader15getLoadingStateEv",
    }

    pub unsafe fn loader_is_patchless(loader: *mut c_void) -> bool {
        win: "?isPatchless@Loader@geode@@QEBA_NXZ",
        mac_intel: "_ZNK5geode6Loader11isPatchlessEv",
        mac_arm: "_ZNK5geode6Loader11isPatchlessEv",
        ios: "_ZNK5geode6Loader11isPatchlessEv",
        android32: "_ZNK5geode6Loader11isPatchlessEv",
        android64: "_ZNK5geode6Loader11isPatchlessEv",
    }

    pub unsafe fn loader_is_mod_installed(loader: *mut c_void, id: StlStringView) -> bool {
        win: "?isModInstalled@Loader@geode@@QEBA_NV?$basic_string_view@DU?$char_traits@D@std@@@std@@@Z",
        mac_intel: "_ZNK5geode6Loader14isModInstalledENSt3__117basic_string_viewIcNS1_11char_traitsIcEEEE",
        mac_arm: "_ZNK5geode6Loader14isModInstalledENSt3__117basic_string_viewIcNS1_11char_traitsIcEEEE",
        ios: "_ZNK5geode6Loader14isModInstalledENSt3__117basic_string_viewIcNS1_11char_traitsIcEEEE",
        android32: "_ZNK5geode6Loader14isModInstalledENSt6__ndk117basic_string_viewIcNS1_11char_traitsIcEEEE",
        android64: "_ZNK5geode6Loader14isModInstalledENSt6__ndk117basic_string_viewIcNS1_11char_traitsIcEEEE",
    }

    pub unsafe fn loader_get_installed_mod(loader: *mut c_void, id: StlStringView) -> *mut c_void {
        win: "?getInstalledMod@Loader@geode@@QEBAPEAVMod@2@V?$basic_string_view@DU?$char_traits@D@std@@@std@@@Z",
        mac_intel: "_ZNK5geode6Loader15getInstalledModENSt3__117basic_string_viewIcNS1_11char_traitsIcEEEE",
        mac_arm: "_ZNK5geode6Loader15getInstalledModENSt3__117basic_string_viewIcNS1_11char_traitsIcEEEE",
        ios: "_ZNK5geode6Loader15getInstalledModENSt3__117basic_string_viewIcNS1_11char_traitsIcEEEE",
        android32: "_ZNK5geode6Loader15getInstalledModENSt6__ndk117basic_string_viewIcNS1_11char_traitsIcEEEE",
        android64: "_ZNK5geode6Loader15getInstalledModENSt6__ndk117basic_string_viewIcNS1_11char_traitsIcEEEE",
    }

    pub unsafe fn loader_is_mod_loaded(loader: *mut c_void, id: StlStringView) -> bool {
        win: "?isModLoaded@Loader@geode@@QEBA_NV?$basic_string_view@DU?$char_traits@D@std@@@std@@@Z",
        mac_intel: "_ZNK5geode6Loader11isModLoadedENSt3__117basic_string_viewIcNS1_11char_traitsIcEEEE",
        mac_arm: "_ZNK5geode6Loader11isModLoadedENSt3__117basic_string_viewIcNS1_11char_traitsIcEEEE",
        ios: "_ZNK5geode6Loader11isModLoadedENSt3__117basic_string_viewIcNS1_11char_traitsIcEEEE",
        android32: "_ZNK5geode6Loader11isModLoadedENSt6__ndk117basic_string_viewIcNS1_11char_traitsIcEEEE",
        android64: "_ZNK5geode6Loader11isModLoadedENSt6__ndk117basic_string_viewIcNS1_11char_traitsIcEEEE",
    }

    pub unsafe fn loader_get_loaded_mod(loader: *mut c_void, id: StlStringView) -> *mut c_void {
        win: "?getLoadedMod@Loader@geode@@QEBAPEAVMod@2@V?$basic_string_view@DU?$char_traits@D@std@@@std@@@Z",
        mac_intel: "_ZNK5geode6Loader12getLoadedModENSt3__117basic_string_viewIcNS1_11char_traitsIcEEEE",
        mac_arm: "_ZNK5geode6Loader12getLoadedModENSt3__117basic_string_viewIcNS1_11char_traitsIcEEEE",
        ios: "_ZNK5geode6Loader12getLoadedModENSt3__117basic_string_viewIcNS1_11char_traitsIcEEEE",
        android32: "_ZNK5geode6Loader12getLoadedModENSt6__ndk117basic_string_viewIcNS1_11char_traitsIcEEEE",
        android64: "_ZNK5geode6Loader12getLoadedModENSt6__ndk117basic_string_viewIcNS1_11char_traitsIcEEEE",
    }

    pub unsafe fn loader_get_all_mods(loader: *mut c_void) -> method_sret StlVector<*mut c_void> {
        win: "?getAllMods@Loader@geode@@QEAA?AV?$vector@PEAVMod@geode@@V?$allocator@PEAVMod@geode@@@std@@@std@@XZ",
        mac_intel: "_ZN5geode6Loader10getAllModsEv",
        mac_arm: "_ZN5geode6Loader10getAllModsEv",
        ios: "_ZN5geode6Loader10getAllModsEv",
        android32: "_ZN5geode6Loader10getAllModsEv",
        android64: "_ZN5geode6Loader10getAllModsEv",
    }

    pub unsafe fn loader_get_launch_argument_names(loader: *mut c_void) -> method_sret StlVector<StlString> {
        win: "?getLaunchArgumentNames@Loader@geode@@QEBA?AV?$vector@V?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@V?$allocator@V?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@@2@@std@@XZ",
        mac_intel: "_ZNK5geode6Loader22getLaunchArgumentNamesEv",
        mac_arm: "_ZNK5geode6Loader22getLaunchArgumentNamesEv",
        ios: "_ZNK5geode6Loader22getLaunchArgumentNamesEv",
        android32: "_ZNK5geode6Loader22getLaunchArgumentNamesEv",
        android64: "_ZNK5geode6Loader22getLaunchArgumentNamesEv",
    }

    pub unsafe fn loader_has_launch_argument(loader: *mut c_void, name: StlStringView) -> bool {
        win: "?hasLaunchArgument@Loader@geode@@QEBA_NV?$basic_string_view@DU?$char_traits@D@std@@@std@@@Z",
        mac_intel: "_ZNK5geode6Loader17hasLaunchArgumentENSt3__117basic_string_viewIcNS1_11char_traitsIcEEEE",
        mac_arm: "_ZNK5geode6Loader17hasLaunchArgumentENSt3__117basic_string_viewIcNS1_11char_traitsIcEEEE",
        ios: "_ZNK5geode6Loader17hasLaunchArgumentENSt3__117basic_string_viewIcNS1_11char_traitsIcEEEE",
        android32: "_ZNK5geode6Loader17hasLaunchArgumentENSt6__ndk117basic_string_viewIcNS1_11char_traitsIcEEEE",
        android64: "_ZNK5geode6Loader17hasLaunchArgumentENSt6__ndk117basic_string_viewIcNS1_11char_traitsIcEEEE",
    }

    pub unsafe fn loader_get_launch_argument(loader: *mut c_void, name: StlStringView) -> method_sret StlOptional<StlString> {
        win: "?getLaunchArgument@Loader@geode@@QEBA?AV?$optional@V?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@@std@@V?$basic_string_view@DU?$char_traits@D@std@@@4@@Z",
        mac_intel: "_ZNK5geode6Loader17getLaunchArgumentENSt3__117basic_string_viewIcNS1_11char_traitsIcEEEE",
        mac_arm: "_ZNK5geode6Loader17getLaunchArgumentENSt3__117basic_string_viewIcNS1_11char_traitsIcEEEE",
        ios: "_ZNK5geode6Loader17getLaunchArgumentENSt3__117basic_string_viewIcNS1_11char_traitsIcEEEE",
        android32: "_ZNK5geode6Loader17getLaunchArgumentENSt6__ndk117basic_string_viewIcNS1_11char_traitsIcEEEE",
        android64: "_ZNK5geode6Loader17getLaunchArgumentENSt6__ndk117basic_string_viewIcNS1_11char_traitsIcEEEE",
    }

    pub unsafe fn loader_get_game_version(loader: *mut c_void) -> method_sret StlString {
        win: "?getGameVersion@Loader@geode@@QEAA?AV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@XZ",
        mac_intel: "_ZN5geode6Loader14getGameVersionEv",
        mac_arm: "_ZN5geode6Loader14getGameVersionEv",
        ios: "_ZN5geode6Loader14getGameVersionEv",
        android32: "_ZN5geode6Loader14getGameVersionEv",
        android64: "_ZN5geode6Loader14getGameVersionEv",
    }

    pub unsafe fn mod_get_id(mod_ptr: *mut c_void) -> method_sret ZStringView {
        win: "?getID@Mod@geode@@QEBA?AV?$BasicZStringView@D@2@XZ",
        mac_intel: "_ZNK5geode3Mod5getIDEv",
        mac_arm: "_ZNK5geode3Mod5getIDEv",
        ios: "_ZNK5geode3Mod5getIDEv",
        android32: "_ZNK5geode3Mod5getIDEv",
        android64: "_ZNK5geode3Mod5getIDEv",
    }

    pub unsafe fn mod_get_name(mod_ptr: *mut c_void) -> method_sret ZStringView {
        win: "?getName@Mod@geode@@QEBA?AV?$BasicZStringView@D@2@XZ",
        mac_intel: "_ZNK5geode3Mod7getNameEv",
        mac_arm: "_ZNK5geode3Mod7getNameEv",
        ios: "_ZNK5geode3Mod7getNameEv",
        android32: "_ZNK5geode3Mod7getNameEv",
        android64: "_ZNK5geode3Mod7getNameEv",
    }

    pub unsafe fn mod_is_loaded(mod_ptr: *mut c_void) -> bool {
        win: "?isLoaded@Mod@geode@@QEBA_NXZ",
        mac_intel: "_ZNK5geode3Mod8isLoadedEv",
        mac_arm: "_ZNK5geode3Mod8isLoadedEv",
        ios: "_ZNK5geode3Mod8isLoadedEv",
        android32: "_ZNK5geode3Mod8isLoadedEv",
        android64: "_ZNK5geode3Mod8isLoadedEv",
    }

    pub unsafe fn mod_is_currently_loading(mod_ptr: *mut c_void) -> bool {
        win: "?isCurrentlyLoading@Mod@geode@@QEBA_NXZ",
        mac_intel: "_ZNK5geode3Mod18isCurrentlyLoadingEv",
        mac_arm: "_ZNK5geode3Mod18isCurrentlyLoadingEv",
        ios: "_ZNK5geode3Mod18isCurrentlyLoadingEv",
        android32: "_ZNK5geode3Mod18isCurrentlyLoadingEv",
        android64: "_ZNK5geode3Mod18isCurrentlyLoadingEv",
    }

    pub unsafe fn mod_has_settings(mod_ptr: *mut c_void) -> bool {
        win: "?hasSettings@Mod@geode@@QEBA_NXZ",
        mac_intel: "_ZNK5geode3Mod11hasSettingsEv",
        mac_arm: "_ZNK5geode3Mod11hasSettingsEv",
        ios: "_ZNK5geode3Mod11hasSettingsEv",
        android32: "_ZNK5geode3Mod11hasSettingsEv",
        android64: "_ZNK5geode3Mod11hasSettingsEv",
    }

    pub unsafe fn mod_get_setting_keys(mod_ptr: *mut c_void) -> method_sret StlVector<StlString> {
        win: "?getSettingKeys@Mod@geode@@QEBA?AV?$vector@V?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@V?$allocator@V?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@@2@@std@@XZ",
        mac_intel: "_ZNK5geode3Mod14getSettingKeysEv",
        mac_arm: "_ZNK5geode3Mod14getSettingKeysEv",
        ios: "_ZNK5geode3Mod14getSettingKeysEv",
        android32: "_ZNK5geode3Mod14getSettingKeysEv",
        android64: "_ZNK5geode3Mod14getSettingKeysEv",
    }

    pub unsafe fn mod_has_setting(mod_ptr: *mut c_void, key: StlStringView) -> bool {
        win: "?hasSetting@Mod@geode@@QEBA_NV?$basic_string_view@DU?$char_traits@D@std@@@std@@@Z",
        mac_intel: "_ZNK5geode3Mod10hasSettingENSt3__117basic_string_viewIcNS1_11char_traitsIcEEEE",
        mac_arm: "_ZNK5geode3Mod10hasSettingENSt3__117basic_string_viewIcNS1_11char_traitsIcEEEE",
        ios: "_ZNK5geode3Mod10hasSettingENSt3__117basic_string_viewIcNS1_11char_traitsIcEEEE",
        android32: "_ZNK5geode3Mod10hasSettingENSt6__ndk117basic_string_viewIcNS1_11char_traitsIcEEEE",
        android64: "_ZNK5geode3Mod10hasSettingENSt6__ndk117basic_string_viewIcNS1_11char_traitsIcEEEE",
    }

    pub unsafe fn mod_get_setting(mod_ptr: *mut c_void, key: StlStringView) -> method_sret StlSharedPtr<c_void> {
        win: "?getSetting@Mod@geode@@QEBA?AV?$shared_ptr@VSettingV3@geode@@@std@@V?$basic_string_view@DU?$char_traits@D@std@@@4@@Z",
        mac_intel: "_ZNK5geode3Mod10getSettingENSt3__117basic_string_viewIcNS1_11char_traitsIcEEEE",
        mac_arm: "_ZNK5geode3Mod10getSettingENSt3__117basic_string_viewIcNS1_11char_traitsIcEEEE",
        ios: "_ZNK5geode3Mod10getSettingENSt3__117basic_string_viewIcNS1_11char_traitsIcEEEE",
        android32: "_ZNK5geode3Mod10getSettingENSt6__ndk117basic_string_viewIcNS1_11char_traitsIcEEEE",
        android64: "_ZNK5geode3Mod10getSettingENSt6__ndk117basic_string_viewIcNS1_11char_traitsIcEEEE",
    }

    pub unsafe fn mod_get_launch_argument_name(mod_ptr: *mut c_void, name: StlStringView) -> method_sret StlString {
        win: "?getLaunchArgumentName@Mod@geode@@QEBA?AV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@V?$basic_string_view@DU?$char_traits@D@std@@@4@@Z",
        mac_intel: "_ZNK5geode3Mod21getLaunchArgumentNameENSt3__117basic_string_viewIcNS1_11char_traitsIcEEEE",
        mac_arm: "_ZNK5geode3Mod21getLaunchArgumentNameENSt3__117basic_string_viewIcNS1_11char_traitsIcEEEE",
        ios: "_ZNK5geode3Mod21getLaunchArgumentNameENSt3__117basic_string_viewIcNS1_11char_traitsIcEEEE",
        android32: "_ZNK5geode3Mod21getLaunchArgumentNameENSt6__ndk117basic_string_viewIcNS1_11char_traitsIcEEEE",
        android64: "_ZNK5geode3Mod21getLaunchArgumentNameENSt6__ndk117basic_string_viewIcNS1_11char_traitsIcEEEE",
    }

    pub unsafe fn mod_get_launch_argument_names(mod_ptr: *mut c_void) -> method_sret StlVector<StlString> {
        win: "?getLaunchArgumentNames@Mod@geode@@QEBA?AV?$vector@V?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@V?$allocator@V?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@@2@@std@@XZ",
        mac_intel: "_ZNK5geode3Mod22getLaunchArgumentNamesEv",
        mac_arm: "_ZNK5geode3Mod22getLaunchArgumentNamesEv",
        ios: "_ZNK5geode3Mod22getLaunchArgumentNamesEv",
        android32: "_ZNK5geode3Mod22getLaunchArgumentNamesEv",
        android64: "_ZNK5geode3Mod22getLaunchArgumentNamesEv",
    }

    pub unsafe fn mod_has_launch_argument(mod_ptr: *mut c_void, name: StlStringView) -> bool {
        win: "?hasLaunchArgument@Mod@geode@@QEBA_NV?$basic_string_view@DU?$char_traits@D@std@@@std@@@Z",
        mac_intel: "_ZNK5geode3Mod17hasLaunchArgumentENSt3__117basic_string_viewIcNS1_11char_traitsIcEEEE",
        mac_arm: "_ZNK5geode3Mod17hasLaunchArgumentENSt3__117basic_string_viewIcNS1_11char_traitsIcEEEE",
        ios: "_ZNK5geode3Mod17hasLaunchArgumentENSt3__117basic_string_viewIcNS1_11char_traitsIcEEEE",
        android32: "_ZNK5geode3Mod17hasLaunchArgumentENSt6__ndk117basic_string_viewIcNS1_11char_traitsIcEEEE",
        android64: "_ZNK5geode3Mod17hasLaunchArgumentENSt6__ndk117basic_string_viewIcNS1_11char_traitsIcEEEE",
    }

    pub unsafe fn mod_get_launch_argument(mod_ptr: *mut c_void, name: StlStringView) -> method_sret StlOptional<StlString> {
        win: "?getLaunchArgument@Mod@geode@@QEBA?AV?$optional@V?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@@std@@V?$basic_string_view@DU?$char_traits@D@std@@@4@@Z",
        mac_intel: "_ZNK5geode3Mod17getLaunchArgumentENSt3__117basic_string_viewIcNS1_11char_traitsIcEEEE",
        mac_arm: "_ZNK5geode3Mod17getLaunchArgumentENSt3__117basic_string_viewIcNS1_11char_traitsIcEEEE",
        ios: "_ZNK5geode3Mod17getLaunchArgumentENSt3__117basic_string_viewIcNS1_11char_traitsIcEEEE",
        android32: "_ZNK5geode3Mod17getLaunchArgumentENSt6__ndk117basic_string_viewIcNS1_11char_traitsIcEEEE",
        android64: "_ZNK5geode3Mod17getLaunchArgumentENSt6__ndk117basic_string_viewIcNS1_11char_traitsIcEEEE",
    }

    pub unsafe fn mod_get_hooks(mod_ptr: *mut c_void) -> method_sret StlVector<*mut c_void> {
        win: "?getHooks@Mod@geode@@QEBA?AV?$vector@PEAVHook@geode@@V?$allocator@PEAVHook@geode@@@std@@@std@@XZ",
        mac_intel: "_ZNK5geode3Mod8getHooksEv",
        mac_arm: "_ZNK5geode3Mod8getHooksEv",
        ios: "_ZNK5geode3Mod8getHooksEv",
        android32: "_ZNK5geode3Mod8getHooksEv",
        android64: "_ZNK5geode3Mod8getHooksEv",
    }

    pub unsafe fn mod_get_patches(mod_ptr: *mut c_void) -> method_sret StlVector<*mut c_void> {
        win: "?getPatches@Mod@geode@@QEBA?AV?$vector@PEAVPatch@geode@@V?$allocator@PEAVPatch@geode@@@std@@@std@@XZ",
        mac_intel: "_ZNK5geode3Mod10getPatchesEv",
        mac_arm: "_ZNK5geode3Mod10getPatchesEv",
        ios: "_ZNK5geode3Mod10getPatchesEv",
        android32: "_ZNK5geode3Mod10getPatchesEv",
        android64: "_ZNK5geode3Mod10getPatchesEv",
    }

    pub unsafe fn mod_enable(mod_ptr: *mut c_void) -> method_sret GeodeResult<()> {
        win: "?enable@Mod@geode@@QEAA?AV?$Result@XV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@@2@XZ",
        mac_intel: "_ZN5geode3Mod6enableEv",
        mac_arm: "_ZN5geode3Mod6enableEv",
        ios: "_ZN5geode3Mod6enableEv",
        android32: "_ZN5geode3Mod6enableEv",
        android64: "_ZN5geode3Mod6enableEv",
    }

    pub unsafe fn mod_disable(mod_ptr: *mut c_void) -> method_sret GeodeResult<()> {
        win: "?disable@Mod@geode@@QEAA?AV?$Result@XV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@@2@XZ",
        mac_intel: "_ZN5geode3Mod7disableEv",
        mac_arm: "_ZN5geode3Mod7disableEv",
        ios: "_ZN5geode3Mod7disableEv",
        android32: "_ZN5geode3Mod7disableEv",
        android64: "_ZN5geode3Mod7disableEv",
    }

    pub unsafe fn mod_settings_manager_from(mod_ptr: *mut c_void) -> *mut c_void {
        win: "?from@ModSettingsManager@geode@@SAPEAV12@PEAVMod@2@@Z",
        mac_intel: "_ZN5geode18ModSettingsManager4fromEPNS_3ModE",
        mac_arm: "_ZN5geode18ModSettingsManager4fromEPNS_3ModE",
        ios: "_ZN5geode18ModSettingsManager4fromEPNS_3ModE",
        android32: "_ZN5geode18ModSettingsManager4fromEPNS_3ModE",
        android64: "_ZN5geode18ModSettingsManager4fromEPNS_3ModE",
    }

    pub unsafe fn mod_settings_manager_get(manager: *mut c_void, key: StlStringView) -> method_sret StlSharedPtr<c_void> {
        win: "?get@ModSettingsManager@geode@@QEAA?AV?$shared_ptr@VSettingV3@geode@@@std@@V?$basic_string_view@DU?$char_traits@D@std@@@4@@Z",
        mac_intel: "_ZN5geode18ModSettingsManager3getENSt3__117basic_string_viewIcNS1_11char_traitsIcEEEE",
        mac_arm: "_ZN5geode18ModSettingsManager3getENSt3__117basic_string_viewIcNS1_11char_traitsIcEEEE",
        ios: "_ZN5geode18ModSettingsManager3getENSt3__117basic_string_viewIcNS1_11char_traitsIcEEEE",
        android32: "_ZN5geode18ModSettingsManager3getENSt6__ndk117basic_string_viewIcNS1_11char_traitsIcEEEE",
        android64: "_ZN5geode18ModSettingsManager3getENSt6__ndk117basic_string_viewIcNS1_11char_traitsIcEEEE",
    }

    pub unsafe fn mod_settings_manager_restart_required(manager: *mut c_void) -> bool {
        win: "?restartRequired@ModSettingsManager@geode@@QEBA_NXZ",
        mac_intel: "_ZNK5geode18ModSettingsManager15restartRequiredEv",
        mac_arm: "_ZNK5geode18ModSettingsManager15restartRequiredEv",
        ios: "_ZNK5geode18ModSettingsManager15restartRequiredEv",
        android32: "_ZNK5geode18ModSettingsManager15restartRequiredEv",
        android64: "_ZNK5geode18ModSettingsManager15restartRequiredEv",
    }

    pub unsafe fn mod_settings_manager_add_dependant(manager: *mut c_void, mod_ptr: *mut c_void) {
        win: "?addDependant@ModSettingsManager@geode@@QEAAXPEAVMod@2@@Z",
        mac_intel: "_ZN5geode18ModSettingsManager12addDependantEPNS_3ModE",
        mac_arm: "_ZN5geode18ModSettingsManager12addDependantEPNS_3ModE",
        ios: "_ZN5geode18ModSettingsManager12addDependantEPNS_3ModE",
        android32: "_ZN5geode18ModSettingsManager12addDependantEPNS_3ModE",
        android64: "_ZN5geode18ModSettingsManager12addDependantEPNS_3ModE",
    }

    pub unsafe fn create_convention(conv: i32) -> sret StlSharedPtr<c_void> {
        win: "?createConvention@hook@geode@@YA?AV?$shared_ptr@VCallingConvention@hook@tulip@@@std@@W4TulipConvention@1tulip@@@Z",
        mac_intel: "_ZN5geode4hook16createConventionEN5tulip4hook15TulipConventionE",
        mac_arm: "_ZN5geode4hook16createConventionEN5tulip4hook15TulipConventionE",
        ios: "_ZN5geode4hook16createConventionEN5tulip4hook15TulipConventionE",
        android32: "_ZN5geode4hook16createConventionEN5tulip4hook15TulipConventionE",
        android64: "_ZN5geode4hook16createConventionEN5tulip4hook15TulipConventionE",
    }

    pub unsafe fn hook_create(address: *mut c_void, detour: *mut c_void, name: *const StlString, handler_meta: *const HandlerMetadata, hook_meta: HookMetadata) -> sret StlSharedPtr<c_void> {
        win: "?create@Hook@geode@@SA?AV?$shared_ptr@VHook@geode@@@std@@PEAX0V?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@4@VHandlerMetadata@hook@tulip@@VHookMetadata@78@@Z",
        mac_intel: "_ZN5geode4Hook6createEPvS1_NSt3__112basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEEN5tulip4hook15HandlerMetadataENSA_12HookMetadataE",
        mac_arm: "_ZN5geode4Hook6createEPvS1_NSt3__112basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEEN5tulip4hook15HandlerMetadataENSA_12HookMetadataE",
        ios: "_ZN5geode4Hook6createEPvS1_NSt3__112basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEEN5tulip4hook15HandlerMetadataENSA_12HookMetadataE",
        android32: "_ZN5geode4Hook6createEPvS1_NSt6__ndk112basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEEN5tulip4hook15HandlerMetadataENSA_12HookMetadataE",
        android64: "_ZN5geode4Hook6createEPvS1_NSt6__ndk112basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEEN5tulip4hook15HandlerMetadataENSA_12HookMetadataE",
    }

    pub unsafe fn mod_claim_hook(mod_ptr: *mut c_void, hook_sptr: *const StlSharedPtr<c_void>) -> method_sret GeodeResult<*mut c_void> {
        win: "?claimHook@Mod@geode@@QEAA?AV?$Result@PEAVHook@geode@@V?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@@2@V?$shared_ptr@VHook@geode@@@std@@@Z",
        mac_intel: "_ZN5geode3Mod9claimHookENSt3__110shared_ptrINS_4HookEEE",
        mac_arm: "_ZN5geode3Mod9claimHookENSt3__110shared_ptrINS_4HookEEE",
        ios: "_ZN5geode3Mod9claimHookENSt3__110shared_ptrINS_4HookEEE",
        android32: "_ZN5geode3Mod9claimHookENSt6__ndk110shared_ptrINS_4HookEEE",
        android64: "_ZN5geode3Mod9claimHookENSt6__ndk110shared_ptrINS_4HookEEE",
    }

    pub unsafe fn hook_get_owner(hook_ptr: *mut c_void) -> *mut c_void {
        win: "?getOwner@Hook@geode@@QEBAPEAVMod@2@XZ",
        mac_intel: "_ZNK5geode4Hook8getOwnerEv",
        mac_arm: "_ZNK5geode4Hook8getOwnerEv",
        ios: "_ZNK5geode4Hook8getOwnerEv",
        android32: "_ZNK5geode4Hook8getOwnerEv",
        android64: "_ZNK5geode4Hook8getOwnerEv",
    }

    pub unsafe fn hook_is_enabled(hook_ptr: *mut c_void) -> bool {
        win: "?isEnabled@Hook@geode@@QEBA_NXZ",
        mac_intel: "_ZNK5geode4Hook9isEnabledEv",
        mac_arm: "_ZNK5geode4Hook9isEnabledEv",
        ios: "_ZNK5geode4Hook9isEnabledEv",
        android32: "_ZNK5geode4Hook9isEnabledEv",
        android64: "_ZNK5geode4Hook9isEnabledEv",
    }

    pub unsafe fn hook_enable(hook_ptr: *mut c_void) -> method_sret GeodeResult<()> {
        win: "?enable@Hook@geode@@QEAA?AV?$Result@XV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@@2@XZ",
        mac_intel: "_ZN5geode4Hook6enableEv",
        mac_arm: "_ZN5geode4Hook6enableEv",
        ios: "_ZN5geode4Hook6enableEv",
        android32: "_ZN5geode4Hook6enableEv",
        android64: "_ZN5geode4Hook6enableEv",
    }

    pub unsafe fn hook_disable(hook_ptr: *mut c_void) -> method_sret GeodeResult<()> {
        win: "?disable@Hook@geode@@QEAA?AV?$Result@XV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@@2@XZ",
        mac_intel: "_ZN5geode4Hook7disableEv",
        mac_arm: "_ZN5geode4Hook7disableEv",
        ios: "_ZN5geode4Hook7disableEv",
        android32: "_ZN5geode4Hook7disableEv",
        android64: "_ZN5geode4Hook7disableEv",
    }

    pub unsafe fn hook_toggle(hook_ptr: *mut c_void) -> method_sret GeodeResult<()> {
        win: "?toggle@Hook@geode@@QEAA?AV?$Result@XV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@@2@XZ",
        mac_intel: "_ZN5geode4Hook6toggleEv",
        mac_arm: "_ZN5geode4Hook6toggleEv",
        ios: "_ZN5geode4Hook6toggleEv",
        android32: "_ZN5geode4Hook6toggleEv",
        android64: "_ZN5geode4Hook6toggleEv",
    }

    pub unsafe fn hook_toggle_to(hook_ptr: *mut c_void, enabled: bool) -> method_sret GeodeResult<()> {
        win: "?toggle@Hook@geode@@QEAA?AV?$Result@XV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@@2@_N@Z",
        mac_intel: "_ZN5geode4Hook6toggleEb",
        mac_arm: "_ZN5geode4Hook6toggleEb",
        ios: "_ZN5geode4Hook6toggleEb",
        android32: "_ZN5geode4Hook6toggleEb",
        android64: "_ZN5geode4Hook6toggleEb",
    }

    pub unsafe fn hook_get_auto_enable(hook_ptr: *mut c_void) -> bool {
        win: "?getAutoEnable@Hook@geode@@QEBA_NXZ",
        mac_intel: "_ZNK5geode4Hook13getAutoEnableEv",
        mac_arm: "_ZNK5geode4Hook13getAutoEnableEv",
        ios: "_ZNK5geode4Hook13getAutoEnableEv",
        android32: "_ZNK5geode4Hook13getAutoEnableEv",
        android64: "_ZNK5geode4Hook13getAutoEnableEv",
    }

    pub unsafe fn hook_set_auto_enable(hook_ptr: *mut c_void, enabled: bool) {
        win: "?setAutoEnable@Hook@geode@@QEAAX_N@Z",
        mac_intel: "_ZN5geode4Hook13setAutoEnableEb",
        mac_arm: "_ZN5geode4Hook13setAutoEnableEb",
        ios: "_ZN5geode4Hook13setAutoEnableEb",
        android32: "_ZN5geode4Hook13setAutoEnableEb",
        android64: "_ZN5geode4Hook13setAutoEnableEb",
    }

    pub unsafe fn hook_get_address(hook_ptr: *mut c_void) -> usize {
        win: "?getAddress@Hook@geode@@QEBA_KXZ",
        mac_intel: "_ZNK5geode4Hook10getAddressEv",
        mac_arm: "_ZNK5geode4Hook10getAddressEv",
        ios: "_ZNK5geode4Hook10getAddressEv",
        android32: "_ZNK5geode4Hook10getAddressEv",
        android64: "_ZNK5geode4Hook10getAddressEv",
    }

    pub unsafe fn hook_get_display_name(hook_ptr: *mut c_void) -> method_sret StlStringView {
        win: "?getDisplayName@Hook@geode@@QEBA?AV?$basic_string_view@DU?$char_traits@D@std@@@std@@XZ",
        mac_intel: "_ZNK5geode4Hook14getDisplayNameEv",
        mac_arm: "_ZNK5geode4Hook14getDisplayNameEv",
        ios: "_ZNK5geode4Hook14getDisplayNameEv",
        android32: "_ZNK5geode4Hook14getDisplayNameEv",
        android64: "_ZNK5geode4Hook14getDisplayNameEv",
    }

    pub unsafe fn hook_get_hook_metadata(hook_ptr: *mut c_void) -> HookMetadata {
        win: "?getHookMetadata@Hook@geode@@QEBA?AVHookMetadata@hook@tulip@@XZ",
        mac_intel: "_ZNK5geode4Hook15getHookMetadataEv",
        mac_arm: "_ZNK5geode4Hook15getHookMetadataEv",
        ios: "_ZNK5geode4Hook15getHookMetadataEv",
        android32: "_ZNK5geode4Hook15getHookMetadataEv",
        android64: "_ZNK5geode4Hook15getHookMetadataEv",
    }

    pub unsafe fn hook_set_hook_metadata(hook_ptr: *mut c_void, metadata: *const HookMetadata) {
        win: "?setHookMetadata@Hook@geode@@QEAAXAEBVHookMetadata@hook@tulip@@@Z",
        mac_intel: "_ZN5geode4Hook15setHookMetadataERKN5tulip4hook12HookMetadataE",
        mac_arm: "_ZN5geode4Hook15setHookMetadataERKN5tulip4hook12HookMetadataE",
        ios: "_ZN5geode4Hook15setHookMetadataERKN5tulip4hook12HookMetadataE",
        android32: "_ZN5geode4Hook15setHookMetadataERKN5tulip4hook12HookMetadataE",
        android64: "_ZN5geode4Hook15setHookMetadataERKN5tulip4hook12HookMetadataE",
    }

    pub unsafe fn hook_get_priority(hook_ptr: *mut c_void) -> i32 {
        win: "?getPriority@Hook@geode@@QEBAHXZ",
        mac_intel: "_ZNK5geode4Hook11getPriorityEv",
        mac_arm: "_ZNK5geode4Hook11getPriorityEv",
        ios: "_ZNK5geode4Hook11getPriorityEv",
        android32: "_ZNK5geode4Hook11getPriorityEv",
        android64: "_ZNK5geode4Hook11getPriorityEv",
    }

    pub unsafe fn hook_set_priority(hook_ptr: *mut c_void, priority: i32) {
        win: "?setPriority@Hook@geode@@QEAAXH@Z",
        mac_intel: "_ZN5geode4Hook11setPriorityEi",
        mac_arm: "_ZN5geode4Hook11setPriorityEi",
        ios: "_ZN5geode4Hook11setPriorityEi",
        android32: "_ZN5geode4Hook11setPriorityEi",
        android64: "_ZN5geode4Hook11setPriorityEi",
    }

    pub unsafe fn patch_create(address: *mut c_void, bytes: *const ByteSpan) -> sret StlSharedPtr<c_void> {
        win: "?create@Patch@geode@@SA?AV?$shared_ptr@VPatch@geode@@@std@@PEAXV?$span@$$CBE$0?0@4@@Z",
        mac_intel: "_ZN5geode5Patch6createEPvNSt3__14spanIKhLm18446744073709551615EEE",
        mac_arm: "_ZN5geode5Patch6createEPvNSt3__14spanIKhLm18446744073709551615EEE",
        ios: "_ZN5geode5Patch6createEPvNSt3__14spanIKhLm18446744073709551615EEE",
        android32: "_ZN5geode5Patch6createEPvNSt6__ndk14spanIKhLj4294967295EEE",
        android64: "_ZN5geode5Patch6createEPvNSt6__ndk14spanIKhLm18446744073709551615EEE",
    }

    pub unsafe fn mod_claim_patch(mod_ptr: *mut c_void, patch_sptr: *const StlSharedPtr<c_void>) -> method_sret GeodeResult<*mut c_void> {
        win: "?claimPatch@Mod@geode@@QEAA?AV?$Result@PEAVPatch@geode@@V?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@@2@V?$shared_ptr@VPatch@geode@@@std@@@Z",
        mac_intel: "_ZN5geode3Mod10claimPatchENSt3__110shared_ptrINS_5PatchEEE",
        mac_arm: "_ZN5geode3Mod10claimPatchENSt3__110shared_ptrINS_5PatchEEE",
        ios: "_ZN5geode3Mod10claimPatchENSt3__110shared_ptrINS_5PatchEEE",
        android32: "_ZN5geode3Mod10claimPatchENSt6__ndk110shared_ptrINS_5PatchEEE",
        android64: "_ZN5geode3Mod10claimPatchENSt6__ndk110shared_ptrINS_5PatchEEE",
    }

    pub unsafe fn patch_get_owner(patch_ptr: *mut c_void) -> *mut c_void {
        win: "?getOwner@Patch@geode@@QEBAPEAVMod@2@XZ",
        mac_intel: "_ZNK5geode5Patch8getOwnerEv",
        mac_arm: "_ZNK5geode5Patch8getOwnerEv",
        ios: "_ZNK5geode5Patch8getOwnerEv",
        android32: "_ZNK5geode5Patch8getOwnerEv",
        android64: "_ZNK5geode5Patch8getOwnerEv",
    }

    pub unsafe fn patch_is_enabled(patch_ptr: *mut c_void) -> bool {
        win: "?isEnabled@Patch@geode@@QEBA_NXZ",
        mac_intel: "_ZNK5geode5Patch9isEnabledEv",
        mac_arm: "_ZNK5geode5Patch9isEnabledEv",
        ios: "_ZNK5geode5Patch9isEnabledEv",
        android32: "_ZNK5geode5Patch9isEnabledEv",
        android64: "_ZNK5geode5Patch9isEnabledEv",
    }

    pub unsafe fn patch_enable(patch_ptr: *mut c_void) -> method_sret GeodeResult<()> {
        win: "?enable@Patch@geode@@QEAA?AV?$Result@XV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@@2@XZ",
        mac_intel: "_ZN5geode5Patch6enableEv",
        mac_arm: "_ZN5geode5Patch6enableEv",
        ios: "_ZN5geode5Patch6enableEv",
        android32: "_ZN5geode5Patch6enableEv",
        android64: "_ZN5geode5Patch6enableEv",
    }

    pub unsafe fn patch_disable(patch_ptr: *mut c_void) -> method_sret GeodeResult<()> {
        win: "?disable@Patch@geode@@QEAA?AV?$Result@XV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@@2@XZ",
        mac_intel: "_ZN5geode5Patch7disableEv",
        mac_arm: "_ZN5geode5Patch7disableEv",
        ios: "_ZN5geode5Patch7disableEv",
        android32: "_ZN5geode5Patch7disableEv",
        android64: "_ZN5geode5Patch7disableEv",
    }

    pub unsafe fn patch_toggle(patch_ptr: *mut c_void) -> method_sret GeodeResult<()> {
        win: "?toggle@Patch@geode@@QEAA?AV?$Result@XV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@@2@XZ",
        mac_intel: "_ZN5geode5Patch6toggleEv",
        mac_arm: "_ZN5geode5Patch6toggleEv",
        ios: "_ZN5geode5Patch6toggleEv",
        android32: "_ZN5geode5Patch6toggleEv",
        android64: "_ZN5geode5Patch6toggleEv",
    }

    pub unsafe fn patch_toggle_to(patch_ptr: *mut c_void, enabled: bool) -> method_sret GeodeResult<()> {
        win: "?toggle@Patch@geode@@QEAA?AV?$Result@XV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@@2@_N@Z",
        mac_intel: "_ZN5geode5Patch6toggleEb",
        mac_arm: "_ZN5geode5Patch6toggleEb",
        ios: "_ZN5geode5Patch6toggleEb",
        android32: "_ZN5geode5Patch6toggleEb",
        android64: "_ZN5geode5Patch6toggleEb",
    }

    pub unsafe fn patch_get_auto_enable(patch_ptr: *mut c_void) -> bool {
        win: "?getAutoEnable@Patch@geode@@QEBA_NXZ",
        mac_intel: "_ZNK5geode5Patch13getAutoEnableEv",
        mac_arm: "_ZNK5geode5Patch13getAutoEnableEv",
        ios: "_ZNK5geode5Patch13getAutoEnableEv",
        android32: "_ZNK5geode5Patch13getAutoEnableEv",
        android64: "_ZNK5geode5Patch13getAutoEnableEv",
    }

    pub unsafe fn patch_set_auto_enable(patch_ptr: *mut c_void, enabled: bool) {
        win: "?setAutoEnable@Patch@geode@@QEAAX_N@Z",
        mac_intel: "_ZN5geode5Patch13setAutoEnableEb",
        mac_arm: "_ZN5geode5Patch13setAutoEnableEb",
        ios: "_ZN5geode5Patch13setAutoEnableEb",
        android32: "_ZN5geode5Patch13setAutoEnableEb",
        android64: "_ZN5geode5Patch13setAutoEnableEb",
    }

    pub unsafe fn patch_get_bytes(patch_ptr: *mut c_void) -> *const StlVector<u8> {
        win: "?getBytes@Patch@geode@@QEBAAEBV?$vector@EV?$allocator@E@std@@@std@@XZ",
        mac_intel: "_ZNK5geode5Patch8getBytesEv",
        mac_arm: "_ZNK5geode5Patch8getBytesEv",
        ios: "_ZNK5geode5Patch8getBytesEv",
        android32: "_ZNK5geode5Patch8getBytesEv",
        android64: "_ZNK5geode5Patch8getBytesEv",
    }

    pub unsafe fn patch_update_bytes(patch_ptr: *mut c_void, bytes: *const ByteSpan) -> method_sret GeodeResult<()> {
        win: "?updateBytes@Patch@geode@@QEAA?AV?$Result@XV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@@2@V?$span@$$CBE$0?0@std@@@Z",
        mac_intel: "_ZN5geode5Patch11updateBytesENSt3__14spanIKhLm18446744073709551615EEE",
        mac_arm: "_ZN5geode5Patch11updateBytesENSt3__14spanIKhLm18446744073709551615EEE",
        ios: "_ZN5geode5Patch11updateBytesENSt3__14spanIKhLm18446744073709551615EEE",
        android32: "_ZN5geode5Patch11updateBytesENSt6__ndk14spanIKhLj4294967295EEE",
        android64: "_ZN5geode5Patch11updateBytesENSt6__ndk14spanIKhLm18446744073709551615EEE",
    }

    pub unsafe fn patch_get_address(patch_ptr: *mut c_void) -> usize {
        win: "?getAddress@Patch@geode@@QEBA_KXZ",
        mac_intel: "_ZNK5geode5Patch10getAddressEv",
        mac_arm: "_ZNK5geode5Patch10getAddressEv",
        ios: "_ZNK5geode5Patch10getAddressEv",
        android32: "_ZNK5geode5Patch10getAddressEv",
        android64: "_ZNK5geode5Patch10getAddressEv",
    }

    pub unsafe fn setting_get_key(setting: *mut c_void) -> method_sret StlString {
        win: "?getKey@SettingV3@geode@@QEBA?AV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@XZ",
        mac_intel: "_ZNK5geode9SettingV36getKeyEv",
        mac_arm: "_ZNK5geode9SettingV36getKeyEv",
        ios: "_ZNK5geode9SettingV36getKeyEv",
        android32: "_ZNK5geode9SettingV36getKeyEv",
        android64: "_ZNK5geode9SettingV36getKeyEv",
    }

    pub unsafe fn setting_get_mod_id(setting: *mut c_void) -> method_sret StlString {
        win: "?getModID@SettingV3@geode@@QEBA?AV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@XZ",
        mac_intel: "_ZNK5geode9SettingV38getModIDEv",
        mac_arm: "_ZNK5geode9SettingV38getModIDEv",
        ios: "_ZNK5geode9SettingV38getModIDEv",
        android32: "_ZNK5geode9SettingV38getModIDEv",
        android64: "_ZNK5geode9SettingV38getModIDEv",
    }

    pub unsafe fn setting_get_mod(setting: *mut c_void) -> *mut c_void {
        win: "?getMod@SettingV3@geode@@QEBAPEAVMod@2@XZ",
        mac_intel: "_ZNK5geode9SettingV36getModEv",
        mac_arm: "_ZNK5geode9SettingV36getModEv",
        ios: "_ZNK5geode9SettingV36getModEv",
        android32: "_ZNK5geode9SettingV36getModEv",
        android64: "_ZNK5geode9SettingV36getModEv",
    }

    pub unsafe fn setting_get_name(setting: *mut c_void) -> method_sret StlOptional<StlString> {
        win: "?getName@SettingV3@geode@@QEBA?AV?$optional@V?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@@std@@XZ",
        mac_intel: "_ZNK5geode9SettingV37getNameEv",
        mac_arm: "_ZNK5geode9SettingV37getNameEv",
        ios: "_ZNK5geode9SettingV37getNameEv",
        android32: "_ZNK5geode9SettingV37getNameEv",
        android64: "_ZNK5geode9SettingV37getNameEv",
    }

    pub unsafe fn setting_get_display_name(setting: *mut c_void) -> method_sret StlString {
        win: "?getDisplayName@SettingV3@geode@@QEBA?AV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@XZ",
        mac_intel: "_ZNK5geode9SettingV314getDisplayNameEv",
        mac_arm: "_ZNK5geode9SettingV314getDisplayNameEv",
        ios: "_ZNK5geode9SettingV314getDisplayNameEv",
        android32: "_ZNK5geode9SettingV314getDisplayNameEv",
        android64: "_ZNK5geode9SettingV314getDisplayNameEv",
    }

    pub unsafe fn setting_get_description(setting: *mut c_void) -> method_sret StlOptional<StlString> {
        win: "?getDescription@SettingV3@geode@@QEBA?AV?$optional@V?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@@std@@XZ",
        mac_intel: "_ZNK5geode9SettingV314getDescriptionEv",
        mac_arm: "_ZNK5geode9SettingV314getDescriptionEv",
        ios: "_ZNK5geode9SettingV314getDescriptionEv",
        android32: "_ZNK5geode9SettingV314getDescriptionEv",
        android64: "_ZNK5geode9SettingV314getDescriptionEv",
    }

    pub unsafe fn setting_get_enable_if(setting: *mut c_void) -> method_sret StlOptional<StlString> {
        win: "?getEnableIf@SettingV3@geode@@QEBA?AV?$optional@V?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@@std@@XZ",
        mac_intel: "_ZNK5geode9SettingV311getEnableIfEv",
        mac_arm: "_ZNK5geode9SettingV311getEnableIfEv",
        ios: "_ZNK5geode9SettingV311getEnableIfEv",
        android32: "_ZNK5geode9SettingV311getEnableIfEv",
        android64: "_ZNK5geode9SettingV311getEnableIfEv",
    }

    pub unsafe fn setting_should_enable(setting: *mut c_void) -> bool {
        win: "?shouldEnable@SettingV3@geode@@QEBA_NXZ",
        mac_intel: "_ZNK5geode9SettingV312shouldEnableEv",
        mac_arm: "_ZNK5geode9SettingV312shouldEnableEv",
        ios: "_ZNK5geode9SettingV312shouldEnableEv",
        android32: "_ZNK5geode9SettingV312shouldEnableEv",
        android64: "_ZNK5geode9SettingV312shouldEnableEv",
    }

    pub unsafe fn setting_get_enable_if_description(setting: *mut c_void) -> method_sret StlOptional<StlString> {
        win: "?getEnableIfDescription@SettingV3@geode@@QEBA?AV?$optional@V?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@@std@@XZ",
        mac_intel: "_ZNK5geode9SettingV322getEnableIfDescriptionEv",
        mac_arm: "_ZNK5geode9SettingV322getEnableIfDescriptionEv",
        ios: "_ZNK5geode9SettingV322getEnableIfDescriptionEv",
        android32: "_ZNK5geode9SettingV322getEnableIfDescriptionEv",
        android64: "_ZNK5geode9SettingV322getEnableIfDescriptionEv",
    }

    pub unsafe fn setting_requires_restart(setting: *mut c_void) -> bool {
        win: "?requiresRestart@SettingV3@geode@@QEBA_NXZ",
        mac_intel: "_ZNK5geode9SettingV315requiresRestartEv",
        mac_arm: "_ZNK5geode9SettingV315requiresRestartEv",
        ios: "_ZNK5geode9SettingV315requiresRestartEv",
        android32: "_ZNK5geode9SettingV315requiresRestartEv",
        android64: "_ZNK5geode9SettingV315requiresRestartEv",
    }

    pub unsafe fn dirs_get_game_dir() -> sret StlPath {
        win: "?getGameDir@dirs@geode@@YA?AVpath@filesystem@std@@XZ",
        mac_intel: "_ZN5geode4dirs10getGameDirEv",
        mac_arm: "_ZN5geode4dirs10getGameDirEv",
        ios: "_ZN5geode4dirs10getGameDirEv",
        android32: "_ZN5geode4dirs10getGameDirEv",
        android64: "_ZN5geode4dirs10getGameDirEv",
    }

    pub unsafe fn dirs_get_save_dir() -> sret StlPath {
        win: "?getSaveDir@dirs@geode@@YA?AVpath@filesystem@std@@XZ",
        mac_intel: "_ZN5geode4dirs10getSaveDirEv",
        mac_arm: "_ZN5geode4dirs10getSaveDirEv",
        ios: "_ZN5geode4dirs10getSaveDirEv",
        android32: "_ZN5geode4dirs10getSaveDirEv",
        android64: "_ZN5geode4dirs10getSaveDirEv",
    }

    pub unsafe fn dirs_get_geode_dir() -> sret StlPath {
        win: "?getGeodeDir@dirs@geode@@YA?AVpath@filesystem@std@@XZ",
        mac_intel: "_ZN5geode4dirs11getGeodeDirEv",
        mac_arm: "_ZN5geode4dirs11getGeodeDirEv",
        ios: "_ZN5geode4dirs11getGeodeDirEv",
        android32: "_ZN5geode4dirs11getGeodeDirEv",
        android64: "_ZN5geode4dirs11getGeodeDirEv",
    }

    pub unsafe fn dirs_get_geode_save_dir() -> sret StlPath {
        win: "?getGeodeSaveDir@dirs@geode@@YA?AVpath@filesystem@std@@XZ",
        mac_intel: "_ZN5geode4dirs15getGeodeSaveDirEv",
        mac_arm: "_ZN5geode4dirs15getGeodeSaveDirEv",
        ios: "_ZN5geode4dirs15getGeodeSaveDirEv",
        android32: "_ZN5geode4dirs15getGeodeSaveDirEv",
        android64: "_ZN5geode4dirs15getGeodeSaveDirEv",
    }

    pub unsafe fn dirs_get_geode_resources_dir() -> sret StlPath {
        win: "?getGeodeResourcesDir@dirs@geode@@YA?AVpath@filesystem@std@@XZ",
        mac_intel: "_ZN5geode4dirs20getGeodeResourcesDirEv",
        mac_arm: "_ZN5geode4dirs20getGeodeResourcesDirEv",
        ios: "_ZN5geode4dirs20getGeodeResourcesDirEv",
        android32: "_ZN5geode4dirs20getGeodeResourcesDirEv",
        android64: "_ZN5geode4dirs20getGeodeResourcesDirEv",
    }

    pub unsafe fn dirs_get_geode_log_dir() -> sret StlPath {
        win: "?getGeodeLogDir@dirs@geode@@YA?AVpath@filesystem@std@@XZ",
        mac_intel: "_ZN5geode4dirs14getGeodeLogDirEv",
        mac_arm: "_ZN5geode4dirs14getGeodeLogDirEv",
        ios: "_ZN5geode4dirs14getGeodeLogDirEv",
        android32: "_ZN5geode4dirs14getGeodeLogDirEv",
        android64: "_ZN5geode4dirs14getGeodeLogDirEv",
    }

    pub unsafe fn dirs_get_temp_dir() -> sret StlPath {
        win: "?getTempDir@dirs@geode@@YA?AVpath@filesystem@std@@XZ",
        mac_intel: "_ZN5geode4dirs10getTempDirEv",
        mac_arm: "_ZN5geode4dirs10getTempDirEv",
        ios: "_ZN5geode4dirs10getTempDirEv",
        android32: "_ZN5geode4dirs10getTempDirEv",
        android64: "_ZN5geode4dirs10getTempDirEv",
    }

    pub unsafe fn dirs_get_mods_dir() -> sret StlPath {
        win: "?getModsDir@dirs@geode@@YA?AVpath@filesystem@std@@XZ",
        mac_intel: "_ZN5geode4dirs10getModsDirEv",
        mac_arm: "_ZN5geode4dirs10getModsDirEv",
        ios: "_ZN5geode4dirs10getModsDirEv",
        android32: "_ZN5geode4dirs10getModsDirEv",
        android64: "_ZN5geode4dirs10getModsDirEv",
    }

    pub unsafe fn dirs_get_mods_save_dir() -> sret StlPath {
        win: "?getModsSaveDir@dirs@geode@@YA?AVpath@filesystem@std@@XZ",
        mac_intel: "_ZN5geode4dirs14getModsSaveDirEv",
        mac_arm: "_ZN5geode4dirs14getModsSaveDirEv",
        ios: "_ZN5geode4dirs14getModsSaveDirEv",
        android32: "_ZN5geode4dirs14getModsSaveDirEv",
        android64: "_ZN5geode4dirs14getModsSaveDirEv",
    }

    pub unsafe fn dirs_get_mod_runtime_dir() -> sret StlPath {
        win: "?getModRuntimeDir@dirs@geode@@YA?AVpath@filesystem@std@@XZ",
        mac_intel: "_ZN5geode4dirs16getModRuntimeDirEv",
        mac_arm: "_ZN5geode4dirs16getModRuntimeDirEv",
        ios: "_ZN5geode4dirs16getModRuntimeDirEv",
        android32: "_ZN5geode4dirs16getModRuntimeDirEv",
        android64: "_ZN5geode4dirs16getModRuntimeDirEv",
    }

    pub unsafe fn dirs_get_mod_config_dir() -> sret StlPath {
        win: "?getModConfigDir@dirs@geode@@YA?AVpath@filesystem@std@@XZ",
        mac_intel: "_ZN5geode4dirs15getModConfigDirEv",
        mac_arm: "_ZN5geode4dirs15getModConfigDirEv",
        ios: "_ZN5geode4dirs15getModConfigDirEv",
        android32: "_ZN5geode4dirs15getModConfigDirEv",
        android64: "_ZN5geode4dirs15getModConfigDirEv",
    }

    pub unsafe fn dirs_get_index_dir() -> sret StlPath {
        win: "?getIndexDir@dirs@geode@@YA?AVpath@filesystem@std@@XZ",
        mac_intel: "_ZN5geode4dirs11getIndexDirEv",
        mac_arm: "_ZN5geode4dirs11getIndexDirEv",
        ios: "_ZN5geode4dirs11getIndexDirEv",
        android32: "_ZN5geode4dirs11getIndexDirEv",
        android64: "_ZN5geode4dirs11getIndexDirEv",
    }

    pub unsafe fn dirs_get_crashlogs_dir() -> sret StlPath {
        win: "?getCrashlogsDir@dirs@geode@@YA?AVpath@filesystem@std@@XZ",
        mac_intel: "_ZN5geode4dirs15getCrashlogsDirEv",
        mac_arm: "_ZN5geode4dirs15getCrashlogsDirEv",
        ios: "_ZN5geode4dirs15getCrashlogsDirEv",
        android32: "_ZN5geode4dirs15getCrashlogsDirEv",
        android64: "_ZN5geode4dirs15getCrashlogsDirEv",
    }

    pub unsafe fn dirs_get_mod_persistent_dir() -> sret StlPath {
        win: "?getModPersistentDir@dirs@geode@@YA?AVpath@filesystem@std@@XZ",
        mac_intel: "_ZN5geode4dirs19getModPersistentDirEv",
        mac_arm: "_ZN5geode4dirs19getModPersistentDirEv",
        ios: "_ZN5geode4dirs19getModPersistentDirEv",
        android32: "_ZN5geode4dirs19getModPersistentDirEv",
        android64: "_ZN5geode4dirs19getModPersistentDirEv",
    }

    pub unsafe fn dirs_get_resources_dir() -> sret StlPath {
        win: "?getResourcesDir@dirs@geode@@YA?AVpath@filesystem@std@@XZ",
        mac_intel: "_ZN5geode4dirs15getResourcesDirEv",
        mac_arm: "_ZN5geode4dirs15getResourcesDirEv",
        ios: "_ZN5geode4dirs15getResourcesDirEv",
        android32: "_ZN5geode4dirs15getResourcesDirEv",
        android64: "_ZN5geode4dirs15getResourcesDirEv",
    }

    pub unsafe fn log_get_current_log_path() -> *const StlPath {
        win: "?getCurrentLogPath@log@geode@@YAAEBVpath@filesystem@std@@XZ",
        mac_intel: "_ZN5geode3log17getCurrentLogPathEv",
        mac_arm: "_ZN5geode3log17getCurrentLogPathEv",
        ios: "_ZN5geode3log17getCurrentLogPathEv",
        android32: "_ZN5geode3log17getCurrentLogPathEv",
        android64: "_ZN5geode3log17getCurrentLogPathEv",
    }

    pub unsafe fn loader_is_forward_compat_mode(loader: *mut c_void) -> bool {
        win: "?isForwardCompatMode@Loader@geode@@QEAA_NXZ",
        mac_intel: "_ZN5geode6Loader19isForwardCompatModeEv",
        mac_arm: "_ZN5geode6Loader19isForwardCompatModeEv",
        ios: "_ZN5geode6Loader19isForwardCompatModeEv",
        android32: "_ZN5geode6Loader19isForwardCompatModeEv",
        android64: "_ZN5geode6Loader19isForwardCompatModeEv",
    }

    pub unsafe fn loader_save_data(loader: *mut c_void) {
        win: "?saveData@Loader@geode@@QEAAXXZ",
        mac_intel: "_ZN5geode6Loader8saveDataEv",
        mac_arm: "_ZN5geode6Loader8saveDataEv",
        ios: "_ZN5geode6Loader8saveDataEv",
        android32: "_ZN5geode6Loader8saveDataEv",
        android64: "_ZN5geode6Loader8saveDataEv",
    }

    pub unsafe fn loader_load_data(loader: *mut c_void) {
        win: "?loadData@Loader@geode@@QEAAXXZ",
        mac_intel: "_ZN5geode6Loader8loadDataEv",
        mac_arm: "_ZN5geode6Loader8loadDataEv",
        ios: "_ZN5geode6Loader8loadDataEv",
        android32: "_ZN5geode6Loader8loadDataEv",
        android64: "_ZN5geode6Loader8loadDataEv",
    }

    pub unsafe fn loader_get_version(loader: *mut c_void) -> method_sret VersionInfo {
        win: "?getVersion@Loader@geode@@QEAA?AVVersionInfo@2@XZ",
        mac_intel: "_ZN5geode6Loader10getVersionEv",
        mac_arm: "_ZN5geode6Loader10getVersionEv",
        ios: "_ZN5geode6Loader10getVersionEv",
        android32: "_ZN5geode6Loader10getVersionEv",
        android64: "_ZN5geode6Loader10getVersionEv",
    }

    pub unsafe fn loader_min_mod_version(loader: *mut c_void) -> method_sret VersionInfo {
        win: "?minModVersion@Loader@geode@@QEAA?AVVersionInfo@2@XZ",
        mac_intel: "_ZN5geode6Loader13minModVersionEv",
        mac_arm: "_ZN5geode6Loader13minModVersionEv",
        ios: "_ZN5geode6Loader13minModVersionEv",
        android32: "_ZN5geode6Loader13minModVersionEv",
        android64: "_ZN5geode6Loader13minModVersionEv",
    }

    pub unsafe fn loader_max_mod_version(loader: *mut c_void) -> method_sret VersionInfo {
        win: "?maxModVersion@Loader@geode@@QEAA?AVVersionInfo@2@XZ",
        mac_intel: "_ZN5geode6Loader13maxModVersionEv",
        mac_arm: "_ZN5geode6Loader13maxModVersionEv",
        ios: "_ZN5geode6Loader13maxModVersionEv",
        android32: "_ZN5geode6Loader13maxModVersionEv",
        android64: "_ZN5geode6Loader13maxModVersionEv",
    }

    pub unsafe fn loader_is_mod_version_supported(loader: *mut c_void, version: *const VersionInfo) -> bool {
        win: "?isModVersionSupported@Loader@geode@@QEAA_NAEBVVersionInfo@2@@Z",
        mac_intel: "_ZN5geode6Loader21isModVersionSupportedERKNS_11VersionInfoE",
        mac_arm: "_ZN5geode6Loader21isModVersionSupportedERKNS_11VersionInfoE",
        ios: "_ZN5geode6Loader21isModVersionSupportedERKNS_11VersionInfoE",
        android32: "_ZN5geode6Loader21isModVersionSupportedERKNS_11VersionInfoE",
        android64: "_ZN5geode6Loader21isModVersionSupportedERKNS_11VersionInfoE",
    }

    pub unsafe fn loader_get_load_problems(loader: *mut c_void) -> method_sret StlVector<LoadProblem> {
        win: "?getLoadProblems@Loader@geode@@QEBA?AV?$vector@ULoadProblem@geode@@V?$allocator@ULoadProblem@geode@@@std@@@std@@XZ",
        mac_intel: "_ZNK5geode6Loader15getLoadProblemsEv",
        mac_arm: "_ZNK5geode6Loader15getLoadProblemsEv",
        ios: "_ZNK5geode6Loader15getLoadProblemsEv",
        android32: "_ZNK5geode6Loader15getLoadProblemsEv",
        android64: "_ZNK5geode6Loader15getLoadProblemsEv",
    }

    pub unsafe fn loader_get_launch_flag(loader: *mut c_void, name: StlStringView) -> bool {
        win: "?getLaunchFlag@Loader@geode@@QEBA_NV?$basic_string_view@DU?$char_traits@D@std@@@std@@@Z",
        mac_intel: "_ZNK5geode6Loader13getLaunchFlagENSt3__117basic_string_viewIcNS1_11char_traitsIcEEEE",
        mac_arm: "_ZNK5geode6Loader13getLaunchFlagENSt3__117basic_string_viewIcNS1_11char_traitsIcEEEE",
        ios: "_ZNK5geode6Loader13getLaunchFlagENSt3__117basic_string_viewIcNS1_11char_traitsIcEEEE",
        android32: "_ZNK5geode6Loader13getLaunchFlagENSt6__ndk117basic_string_viewIcNS1_11char_traitsIcEEEE",
        android64: "_ZNK5geode6Loader13getLaunchFlagENSt6__ndk117basic_string_viewIcNS1_11char_traitsIcEEEE",
    }

    pub unsafe fn mod_get_developers(mod_ptr: *mut c_void) -> method_sret StlVector<StlString> {
        win: "?getDevelopers@Mod@geode@@QEBA?AV?$vector@V?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@V?$allocator@V?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@@2@@std@@XZ",
        mac_intel: "_ZNK5geode3Mod13getDevelopersEv",
        mac_arm: "_ZNK5geode3Mod13getDevelopersEv",
        ios: "_ZNK5geode3Mod13getDevelopersEv",
        android32: "_ZNK5geode3Mod13getDevelopersEv",
        android64: "_ZNK5geode3Mod13getDevelopersEv",
    }

    pub unsafe fn mod_get_description(mod_ptr: *mut c_void) -> method_sret StlOptional<StlString> {
        win: "?getDescription@Mod@geode@@QEBA?AV?$optional@V?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@@std@@XZ",
        mac_intel: "_ZNK5geode3Mod14getDescriptionEv",
        mac_arm: "_ZNK5geode3Mod14getDescriptionEv",
        ios: "_ZNK5geode3Mod14getDescriptionEv",
        android32: "_ZNK5geode3Mod14getDescriptionEv",
        android64: "_ZNK5geode3Mod14getDescriptionEv",
    }

    pub unsafe fn mod_get_details(mod_ptr: *mut c_void) -> method_sret StlOptional<StlString> {
        win: "?getDetails@Mod@geode@@QEBA?AV?$optional@V?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@@std@@XZ",
        mac_intel: "_ZNK5geode3Mod10getDetailsEv",
        mac_arm: "_ZNK5geode3Mod10getDetailsEv",
        ios: "_ZNK5geode3Mod10getDetailsEv",
        android32: "_ZNK5geode3Mod10getDetailsEv",
        android64: "_ZNK5geode3Mod10getDetailsEv",
    }

    pub unsafe fn mod_get_package_path(mod_ptr: *mut c_void) -> method_sret StlPath {
        win: "?getPackagePath@Mod@geode@@QEBA?AVpath@filesystem@std@@XZ",
        mac_intel: "_ZNK5geode3Mod14getPackagePathEv",
        mac_arm: "_ZNK5geode3Mod14getPackagePathEv",
        ios: "_ZNK5geode3Mod14getPackagePathEv",
        android32: "_ZNK5geode3Mod14getPackagePathEv",
        android64: "_ZNK5geode3Mod14getPackagePathEv",
    }

    pub unsafe fn mod_get_version(mod_ptr: *mut c_void) -> method_sret VersionInfo {
        win: "?getVersion@Mod@geode@@QEBA?AVVersionInfo@2@XZ",
        mac_intel: "_ZNK5geode3Mod10getVersionEv",
        mac_arm: "_ZNK5geode3Mod10getVersionEv",
        ios: "_ZNK5geode3Mod10getVersionEv",
        android32: "_ZNK5geode3Mod10getVersionEv",
        android64: "_ZNK5geode3Mod10getVersionEv",
    }

    pub unsafe fn mod_is_or_will_be_enabled(mod_ptr: *mut c_void) -> bool {
        win: "?isOrWillBeEnabled@Mod@geode@@QEBA_NXZ",
        mac_intel: "_ZNK5geode3Mod17isOrWillBeEnabledEv",
        mac_arm: "_ZNK5geode3Mod17isOrWillBeEnabledEv",
        ios: "_ZNK5geode3Mod17isOrWillBeEnabledEv",
        android32: "_ZNK5geode3Mod17isOrWillBeEnabledEv",
        android64: "_ZNK5geode3Mod17isOrWillBeEnabledEv",
    }

    pub unsafe fn mod_is_internal(mod_ptr: *mut c_void) -> bool {
        win: "?isInternal@Mod@geode@@QEBA_NXZ",
        mac_intel: "_ZNK5geode3Mod10isInternalEv",
        mac_arm: "_ZNK5geode3Mod10isInternalEv",
        ios: "_ZNK5geode3Mod10isInternalEv",
        android32: "_ZNK5geode3Mod10isInternalEv",
        android64: "_ZNK5geode3Mod10isInternalEv",
    }

    pub unsafe fn mod_needs_early_load(mod_ptr: *mut c_void) -> bool {
        win: "?needsEarlyLoad@Mod@geode@@QEBA_NXZ",
        mac_intel: "_ZNK5geode3Mod14needsEarlyLoadEv",
        mac_arm: "_ZNK5geode3Mod14needsEarlyLoadEv",
        ios: "_ZNK5geode3Mod14needsEarlyLoadEv",
        android32: "_ZNK5geode3Mod14needsEarlyLoadEv",
        android64: "_ZNK5geode3Mod14needsEarlyLoadEv",
    }

    pub unsafe fn mod_get_metadata(mod_ptr: *mut c_void) -> *const ModMetadata {
        win: "?getMetadata@Mod@geode@@QEBAAEBVModMetadata@2@XZ",
        mac_intel: "_ZNK5geode3Mod11getMetadataEv",
        mac_arm: "_ZNK5geode3Mod11getMetadataEv",
        ios: "_ZNK5geode3Mod11getMetadataEv",
        android32: "_ZNK5geode3Mod11getMetadataEv",
        android64: "_ZNK5geode3Mod11getMetadataEv",
    }

    pub unsafe fn mod_get_temp_dir(mod_ptr: *mut c_void) -> method_sret StlPath {
        win: "?getTempDir@Mod@geode@@QEBA?AVpath@filesystem@std@@XZ",
        mac_intel: "_ZNK5geode3Mod10getTempDirEv",
        mac_arm: "_ZNK5geode3Mod10getTempDirEv",
        ios: "_ZNK5geode3Mod10getTempDirEv",
        android32: "_ZNK5geode3Mod10getTempDirEv",
        android64: "_ZNK5geode3Mod10getTempDirEv",
    }

    pub unsafe fn mod_get_binary_path(mod_ptr: *mut c_void) -> method_sret StlPath {
        win: "?getBinaryPath@Mod@geode@@QEBA?AVpath@filesystem@std@@XZ",
        mac_intel: "_ZNK5geode3Mod13getBinaryPathEv",
        mac_arm: "_ZNK5geode3Mod13getBinaryPathEv",
        ios: "_ZNK5geode3Mod13getBinaryPathEv",
        android32: "_ZNK5geode3Mod13getBinaryPathEv",
        android64: "_ZNK5geode3Mod13getBinaryPathEv",
    }

    pub unsafe fn mod_get_resources_dir(mod_ptr: *mut c_void) -> method_sret StlPath {
        win: "?getResourcesDir@Mod@geode@@QEBA?AVpath@filesystem@std@@XZ",
        mac_intel: "_ZNK5geode3Mod15getResourcesDirEv",
        mac_arm: "_ZNK5geode3Mod15getResourcesDirEv",
        ios: "_ZNK5geode3Mod15getResourcesDirEv",
        android32: "_ZNK5geode3Mod15getResourcesDirEv",
        android64: "_ZNK5geode3Mod15getResourcesDirEv",
    }

    pub unsafe fn mod_get_dependency_settings_for(mod_ptr: *mut c_void, dependency_id: StlStringView) -> method_sret MatJsonValue {
        win: "?getDependencySettingsFor@Mod@geode@@QEBA?AVValue@matjson@@V?$basic_string_view@DU?$char_traits@D@std@@@std@@@Z",
        mac_intel: "_ZNK5geode3Mod24getDependencySettingsForENSt3__117basic_string_viewIcNS1_11char_traitsIcEEEE",
        mac_arm: "_ZNK5geode3Mod24getDependencySettingsForENSt3__117basic_string_viewIcNS1_11char_traitsIcEEEE",
        ios: "_ZNK5geode3Mod24getDependencySettingsForENSt3__117basic_string_viewIcNS1_11char_traitsIcEEEE",
        android32: "_ZNK5geode3Mod24getDependencySettingsForENSt6__ndk117basic_string_viewIcNS1_11char_traitsIcEEEE",
        android64: "_ZNK5geode3Mod24getDependencySettingsForENSt6__ndk117basic_string_viewIcNS1_11char_traitsIcEEEE",
    }

    pub unsafe fn mod_save_data_result(mod_ptr: *mut c_void) -> method_sret GeodeResult<()> {
        win: "?saveData@Mod@geode@@QEAA?AV?$Result@XV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@@2@XZ",
        mac_intel: "_ZN5geode3Mod8saveDataEv",
        mac_arm: "_ZN5geode3Mod8saveDataEv",
        ios: "_ZN5geode3Mod8saveDataEv",
        android32: "_ZN5geode3Mod8saveDataEv",
        android64: "_ZN5geode3Mod8saveDataEv",
    }

    pub unsafe fn mod_load_data_result(mod_ptr: *mut c_void) -> method_sret GeodeResult<()> {
        win: "?loadData@Mod@geode@@QEAA?AV?$Result@XV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@@2@XZ",
        mac_intel: "_ZN5geode3Mod8loadDataEv",
        mac_arm: "_ZN5geode3Mod8loadDataEv",
        ios: "_ZN5geode3Mod8loadDataEv",
        android32: "_ZN5geode3Mod8loadDataEv",
        android64: "_ZN5geode3Mod8loadDataEv",
    }

    pub unsafe fn mod_get_save_dir(mod_ptr: *mut c_void) -> method_sret StlPath {
        win: "?getSaveDir@Mod@geode@@QEBA?AVpath@filesystem@std@@XZ",
        mac_intel: "_ZNK5geode3Mod10getSaveDirEv",
        mac_arm: "_ZNK5geode3Mod10getSaveDirEv",
        ios: "_ZNK5geode3Mod10getSaveDirEv",
        android32: "_ZNK5geode3Mod10getSaveDirEv",
        android64: "_ZNK5geode3Mod10getSaveDirEv",
    }

    pub unsafe fn mod_get_config_dir(mod_ptr: *mut c_void, create: bool) -> method_sret StlPath {
        win: "?getConfigDir@Mod@geode@@QEBA?AVpath@filesystem@std@@_N@Z",
        mac_intel: "_ZNK5geode3Mod12getConfigDirEb",
        mac_arm: "_ZNK5geode3Mod12getConfigDirEb",
        ios: "_ZNK5geode3Mod12getConfigDirEb",
        android32: "_ZNK5geode3Mod12getConfigDirEb",
        android64: "_ZNK5geode3Mod12getConfigDirEb",
    }

    pub unsafe fn mod_get_persistent_dir(mod_ptr: *mut c_void, create: bool) -> method_sret StlPath {
        win: "?getPersistentDir@Mod@geode@@QEBA?AVpath@filesystem@std@@_N@Z",
        mac_intel: "_ZNK5geode3Mod16getPersistentDirEb",
        mac_arm: "_ZNK5geode3Mod16getPersistentDirEb",
        ios: "_ZNK5geode3Mod16getPersistentDirEb",
        android32: "_ZNK5geode3Mod16getPersistentDirEb",
        android64: "_ZNK5geode3Mod16getPersistentDirEb",
    }

    pub unsafe fn mod_get_launch_flag(mod_ptr: *mut c_void, name: StlStringView) -> bool {
        win: "?getLaunchFlag@Mod@geode@@QEBA_NV?$basic_string_view@DU?$char_traits@D@std@@@std@@@Z",
        mac_intel: "_ZNK5geode3Mod13getLaunchFlagENSt3__117basic_string_viewIcNS1_11char_traitsIcEEEE",
        mac_arm: "_ZNK5geode3Mod13getLaunchFlagENSt3__117basic_string_viewIcNS1_11char_traitsIcEEEE",
        ios: "_ZNK5geode3Mod13getLaunchFlagENSt3__117basic_string_viewIcNS1_11char_traitsIcEEEE",
        android32: "_ZNK5geode3Mod13getLaunchFlagENSt6__ndk117basic_string_viewIcNS1_11char_traitsIcEEEE",
        android64: "_ZNK5geode3Mod13getLaunchFlagENSt6__ndk117basic_string_viewIcNS1_11char_traitsIcEEEE",
    }

    pub unsafe fn mod_uninstall(mod_ptr: *mut c_void, delete_save_data: bool) -> method_sret GeodeResult<()> {
        win: "?uninstall@Mod@geode@@QEAA?AV?$Result@XV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@@2@_N@Z",
        mac_intel: "_ZN5geode3Mod9uninstallEb",
        mac_arm: "_ZN5geode3Mod9uninstallEb",
        ios: "_ZN5geode3Mod9uninstallEb",
        android32: "_ZN5geode3Mod9uninstallEb",
        android64: "_ZN5geode3Mod9uninstallEb",
    }

    pub unsafe fn mod_is_uninstalled(mod_ptr: *mut c_void) -> bool {
        win: "?isUninstalled@Mod@geode@@QEBA_NXZ",
        mac_intel: "_ZNK5geode3Mod13isUninstalledEv",
        mac_arm: "_ZNK5geode3Mod13isUninstalledEv",
        ios: "_ZNK5geode3Mod13isUninstalledEv",
        android32: "_ZNK5geode3Mod13isUninstalledEv",
        android64: "_ZNK5geode3Mod13isUninstalledEv",
    }

    pub unsafe fn mod_get_requested_action(mod_ptr: *mut c_void) -> ModRequestedAction {
        win: "?getRequestedAction@Mod@geode@@QEBA?AW4ModRequestedAction@2@XZ",
        mac_intel: "_ZNK5geode3Mod18getRequestedActionEv",
        mac_arm: "_ZNK5geode3Mod18getRequestedActionEv",
        ios: "_ZNK5geode3Mod18getRequestedActionEv",
        android32: "_ZNK5geode3Mod18getRequestedActionEv",
        android64: "_ZNK5geode3Mod18getRequestedActionEv",
    }

    pub unsafe fn mod_depends(mod_ptr: *mut c_void, id: StlStringView) -> bool {
        win: "?depends@Mod@geode@@QEBA_NV?$basic_string_view@DU?$char_traits@D@std@@@std@@@Z",
        mac_intel: "_ZNK5geode3Mod7dependsENSt3__117basic_string_viewIcNS1_11char_traitsIcEEEE",
        mac_arm: "_ZNK5geode3Mod7dependsENSt3__117basic_string_viewIcNS1_11char_traitsIcEEEE",
        ios: "_ZNK5geode3Mod7dependsENSt3__117basic_string_viewIcNS1_11char_traitsIcEEEE",
        android32: "_ZNK5geode3Mod7dependsENSt6__ndk117basic_string_viewIcNS1_11char_traitsIcEEEE",
        android64: "_ZNK5geode3Mod7dependsENSt6__ndk117basic_string_viewIcNS1_11char_traitsIcEEEE",
    }

    pub unsafe fn mod_has_unresolved_dependencies(mod_ptr: *mut c_void) -> bool {
        win: "?hasUnresolvedDependencies@Mod@geode@@QEBA_NXZ",
        mac_intel: "_ZNK5geode3Mod25hasUnresolvedDependenciesEv",
        mac_arm: "_ZNK5geode3Mod25hasUnresolvedDependenciesEv",
        ios: "_ZNK5geode3Mod25hasUnresolvedDependenciesEv",
        android32: "_ZNK5geode3Mod25hasUnresolvedDependenciesEv",
        android64: "_ZNK5geode3Mod25hasUnresolvedDependenciesEv",
    }

    pub unsafe fn mod_has_unresolved_incompatibilities(mod_ptr: *mut c_void) -> bool {
        win: "?hasUnresolvedIncompatibilities@Mod@geode@@QEBA_NXZ",
        mac_intel: "_ZNK5geode3Mod30hasUnresolvedIncompatibilitiesEv",
        mac_arm: "_ZNK5geode3Mod30hasUnresolvedIncompatibilitiesEv",
        ios: "_ZNK5geode3Mod30hasUnresolvedIncompatibilitiesEv",
        android32: "_ZNK5geode3Mod30hasUnresolvedIncompatibilitiesEv",
        android64: "_ZNK5geode3Mod30hasUnresolvedIncompatibilitiesEv",
    }

    pub unsafe fn mod_expand_sprite_name(mod_ptr: *mut c_void, name: StlStringView) -> method_sret StlString {
        win: "?expandSpriteName@Mod@geode@@QEAA?AV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@V?$basic_string_view@DU?$char_traits@D@std@@@4@@Z",
        mac_intel: "_ZN5geode3Mod16expandSpriteNameENSt3__117basic_string_viewIcNS1_11char_traitsIcEEEE",
        mac_arm: "_ZN5geode3Mod16expandSpriteNameENSt3__117basic_string_viewIcNS1_11char_traitsIcEEEE",
        ios: "_ZN5geode3Mod16expandSpriteNameENSt3__117basic_string_viewIcNS1_11char_traitsIcEEEE",
        android32: "_ZN5geode3Mod16expandSpriteNameENSt6__ndk117basic_string_viewIcNS1_11char_traitsIcEEEE",
        android64: "_ZN5geode3Mod16expandSpriteNameENSt6__ndk117basic_string_viewIcNS1_11char_traitsIcEEEE",
    }

    pub unsafe fn mod_get_runtime_info(mod_ptr: *mut c_void) -> method_sret MatJsonValue {
        win: "?getRuntimeInfo@Mod@geode@@QEBA?AVValue@matjson@@XZ",
        mac_intel: "_ZNK5geode3Mod14getRuntimeInfoEv",
        mac_arm: "_ZNK5geode3Mod14getRuntimeInfoEv",
        ios: "_ZNK5geode3Mod14getRuntimeInfoEv",
        android32: "_ZNK5geode3Mod14getRuntimeInfoEv",
        android64: "_ZNK5geode3Mod14getRuntimeInfoEv",
    }

    pub unsafe fn mod_is_logging_enabled(mod_ptr: *mut c_void) -> bool {
        win: "?isLoggingEnabled@Mod@geode@@QEBA_NXZ",
        mac_intel: "_ZNK5geode3Mod16isLoggingEnabledEv",
        mac_arm: "_ZNK5geode3Mod16isLoggingEnabledEv",
        ios: "_ZNK5geode3Mod16isLoggingEnabledEv",
        android32: "_ZNK5geode3Mod16isLoggingEnabledEv",
        android64: "_ZNK5geode3Mod16isLoggingEnabledEv",
    }

    pub unsafe fn mod_set_logging_enabled(mod_ptr: *mut c_void, enabled: bool) {
        win: "?setLoggingEnabled@Mod@geode@@QEAAX_N@Z",
        mac_intel: "_ZN5geode3Mod17setLoggingEnabledEb",
        mac_arm: "_ZN5geode3Mod17setLoggingEnabledEb",
        ios: "_ZN5geode3Mod17setLoggingEnabledEb",
        android32: "_ZN5geode3Mod17setLoggingEnabledEb",
        android64: "_ZN5geode3Mod17setLoggingEnabledEb",
    }

    pub unsafe fn mod_get_log_level(mod_ptr: *mut c_void) -> Severity {
        win: "?getLogLevel@Mod@geode@@QEBA?AUSeverity@2@XZ",
        mac_intel: "_ZNK5geode3Mod11getLogLevelEv",
        mac_arm: "_ZNK5geode3Mod11getLogLevelEv",
        ios: "_ZNK5geode3Mod11getLogLevelEv",
        android32: "_ZNK5geode3Mod11getLogLevelEv",
        android64: "_ZNK5geode3Mod11getLogLevelEv",
    }

    pub unsafe fn mod_set_log_level(mod_ptr: *mut c_void, level: Severity) {
        win: "?setLogLevel@Mod@geode@@QEAAXUSeverity@2@@Z",
        mac_intel: "_ZN5geode3Mod11setLogLevelENS_8SeverityE",
        mac_arm: "_ZN5geode3Mod11setLogLevelENS_8SeverityE",
        ios: "_ZN5geode3Mod11setLogLevelENS_8SeverityE",
        android32: "_ZN5geode3Mod11setLogLevelENS_8SeverityE",
        android64: "_ZN5geode3Mod11setLogLevelENS_8SeverityE",
    }

    pub unsafe fn mod_targets_outdated_version(mod_ptr: *mut c_void) -> method_sret StlOptional<LoadProblem> {
        win: "?targetsOutdatedVersion@Mod@geode@@QEBA?AV?$optional@ULoadProblem@geode@@@std@@XZ",
        mac_intel: "_ZNK5geode3Mod22targetsOutdatedVersionEv",
        mac_arm: "_ZNK5geode3Mod22targetsOutdatedVersionEv",
        ios: "_ZNK5geode3Mod22targetsOutdatedVersionEv",
        android32: "_ZNK5geode3Mod22targetsOutdatedVersionEv",
        android64: "_ZNK5geode3Mod22targetsOutdatedVersionEv",
    }

    pub unsafe fn mod_failed_to_load(mod_ptr: *mut c_void) -> method_sret StlOptional<LoadProblem> {
        win: "?failedToLoad@Mod@geode@@QEBA?AV?$optional@ULoadProblem@geode@@@std@@XZ",
        mac_intel: "_ZNK5geode3Mod12failedToLoadEv",
        mac_arm: "_ZNK5geode3Mod12failedToLoadEv",
        ios: "_ZNK5geode3Mod12failedToLoadEv",
        android32: "_ZNK5geode3Mod12failedToLoadEv",
        android64: "_ZNK5geode3Mod12failedToLoadEv",
    }

    pub unsafe fn mod_get_load_problem(mod_ptr: *mut c_void) -> method_sret StlOptional<LoadProblem> {
        win: "?getLoadProblem@Mod@geode@@QEBA?AV?$optional@ULoadProblem@geode@@@std@@XZ",
        mac_intel: "_ZNK5geode3Mod14getLoadProblemEv",
        mac_arm: "_ZNK5geode3Mod14getLoadProblemEv",
        ios: "_ZNK5geode3Mod14getLoadProblemEv",
        android32: "_ZNK5geode3Mod14getLoadProblemEv",
        android64: "_ZNK5geode3Mod14getLoadProblemEv",
    }

    pub unsafe fn mod_should_load(mod_ptr: *mut c_void) -> bool {
        win: "?shouldLoad@Mod@geode@@QEBA_NXZ",
        mac_intel: "_ZNK5geode3Mod10shouldLoadEv",
        mac_arm: "_ZNK5geode3Mod10shouldLoadEv",
        ios: "_ZNK5geode3Mod10shouldLoadEv",
        android32: "_ZNK5geode3Mod10shouldLoadEv",
        android64: "_ZNK5geode3Mod10shouldLoadEv",
    }

    pub unsafe fn mod_get_load_priority(mod_ptr: *mut c_void) -> i32 {
        win: "?getLoadPriority@Mod@geode@@QEBAHXZ",
        mac_intel: "_ZNK5geode3Mod15getLoadPriorityEv",
        mac_arm: "_ZNK5geode3Mod15getLoadPriorityEv",
        ios: "_ZNK5geode3Mod15getLoadPriorityEv",
        android32: "_ZNK5geode3Mod15getLoadPriorityEv",
        android64: "_ZNK5geode3Mod15getLoadPriorityEv",
    }

    pub unsafe fn mod_is_pinned(mod_ptr: *mut c_void) -> bool {
        win: "?isPinned@Mod@geode@@QEBA_NXZ",
        mac_intel: "_ZNK5geode3Mod8isPinnedEv",
        mac_arm: "_ZNK5geode3Mod8isPinnedEv",
        ios: "_ZNK5geode3Mod8isPinnedEv",
        android32: "_ZNK5geode3Mod8isPinnedEv",
        android64: "_ZNK5geode3Mod8isPinnedEv",
    }

    pub unsafe fn mod_set_pinned(mod_ptr: *mut c_void, pinned: bool) {
        win: "?setPinned@Mod@geode@@QEAAX_N@Z",
        mac_intel: "_ZN5geode3Mod9setPinnedEb",
        mac_arm: "_ZN5geode3Mod9setPinnedEb",
        ios: "_ZN5geode3Mod9setPinnedEb",
        android32: "_ZN5geode3Mod9setPinnedEb",
        android64: "_ZN5geode3Mod9setPinnedEb",
    }

    pub unsafe fn matjson_value_copy_ctor(this: *mut MatJsonValue, other: *const MatJsonValue) {
        win: "??0Value@matjson@@QEAA@AEBV01@@Z",
        mac_intel: "_ZN7matjson5ValueC1ERKS0_",
        mac_arm: "_ZN7matjson5ValueC1ERKS0_",
        ios: "_ZN7matjson5ValueC1ERKS0_",
        android32: "_ZN7matjson5ValueC1ERKS0_",
        android64: "_ZN7matjson5ValueC1ERKS0_",
    }

    pub unsafe fn matjson_value_dtor(this: *mut MatJsonValue) {
        win: "??1Value@matjson@@QEAA@XZ",
        mac_intel: "_ZN7matjson5ValueD1Ev",
        mac_arm: "_ZN7matjson5ValueD1Ev",
        ios: "_ZN7matjson5ValueD1Ev",
        android32: "_ZN7matjson5ValueD1Ev",
        android64: "_ZN7matjson5ValueD1Ev",
    }

    pub unsafe fn matjson_value_dump(value: *const MatJsonValue, indentation: i32) -> method_sret StlString {
        win: "?dump@Value@matjson@@QEBA?AV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@H@Z",
        mac_intel: "_ZNK7matjson5Value4dumpEi",
        mac_arm: "_ZNK7matjson5Value4dumpEi",
        ios: "_ZNK7matjson5Value4dumpEi",
        android32: "_ZNK7matjson5Value4dumpEi",
        android64: "_ZNK7matjson5Value4dumpEi",
    }

    pub unsafe fn matjson_value_type(value: *const MatJsonValue) -> MatJsonType {
        win: "?type@Value@matjson@@QEBA?AW4Type@2@XZ",
        mac_intel: "_ZNK7matjson5Value4typeEv",
        mac_arm: "_ZNK7matjson5Value4typeEv",
        ios: "_ZNK7matjson5Value4typeEv",
        android32: "_ZNK7matjson5Value4typeEv",
        android64: "_ZNK7matjson5Value4typeEv",
    }

    pub unsafe fn matjson_value_size(value: *const MatJsonValue) -> usize {
        win: "?size@Value@matjson@@QEBA_KXZ",
        mac_intel: "_ZNK7matjson5Value4sizeEv",
        mac_arm: "_ZNK7matjson5Value4sizeEv",
        ios: "_ZNK7matjson5Value4sizeEv",
        android32: "_ZNK7matjson5Value4sizeEv",
        android64: "_ZNK7matjson5Value4sizeEv",
    }

    pub unsafe fn matjson_value_contains(value: *const MatJsonValue, key: StlStringView) -> bool {
        win: "?contains@Value@matjson@@QEBA_NV?$basic_string_view@DU?$char_traits@D@std@@@std@@@Z",
        mac_intel: "_ZNK7matjson5Value8containsENSt3__117basic_string_viewIcNS1_11char_traitsIcEEEE",
        mac_arm: "_ZNK7matjson5Value8containsENSt3__117basic_string_viewIcNS1_11char_traitsIcEEEE",
        ios: "_ZNK7matjson5Value8containsENSt3__117basic_string_viewIcNS1_11char_traitsIcEEEE",
        android32: "_ZNK7matjson5Value8containsENSt6__ndk117basic_string_viewIcNS1_11char_traitsIcEEEE",
        android64: "_ZNK7matjson5Value8containsENSt6__ndk117basic_string_viewIcNS1_11char_traitsIcEEEE",
    }

    pub unsafe fn matjson_value_as_string(value: *const MatJsonValue) -> method_sret GeodeResult<StlString> {
        win: "?asString@Value@matjson@@QEBA?AV?$Result@V?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@V12@@geode@@XZ",
        mac_intel: "_ZNK7matjson5Value8asStringEv",
        mac_arm: "_ZNK7matjson5Value8asStringEv",
        ios: "_ZNK7matjson5Value8asStringEv",
        android32: "_ZNK7matjson5Value8asStringEv",
        android64: "_ZNK7matjson5Value8asStringEv",
    }

    pub unsafe fn mod_metadata_copy_ctor(this: *mut ModMetadata, other: *const ModMetadata) {
        win: "??0ModMetadata@geode@@QEAA@AEBV01@@Z",
        mac_intel: "_ZN5geode11ModMetadataC1ERKS0_",
        mac_arm: "_ZN5geode11ModMetadataC1ERKS0_",
        ios: "_ZN5geode11ModMetadataC1ERKS0_",
        android32: "_ZN5geode11ModMetadataC1ERKS0_",
        android64: "_ZN5geode11ModMetadataC1ERKS0_",
    }

    pub unsafe fn mod_metadata_dtor(this: *mut ModMetadata) {
        win: "??1ModMetadata@geode@@QEAA@XZ",
        mac_intel: "_ZN5geode11ModMetadataD1Ev",
        mac_arm: "_ZN5geode11ModMetadataD1Ev",
        ios: "_ZN5geode11ModMetadataD1Ev",
        android32: "_ZN5geode11ModMetadataD1Ev",
        android64: "_ZN5geode11ModMetadataD1Ev",
    }

    pub unsafe fn mod_metadata_links_copy_ctor(this: *mut ModMetadataLinks, other: *const ModMetadataLinks) {
        win: "??0ModMetadataLinks@geode@@QEAA@AEBV01@@Z",
        mac_intel: "_ZN5geode16ModMetadataLinksC1ERKS0_",
        mac_arm: "_ZN5geode16ModMetadataLinksC1ERKS0_",
        ios: "_ZN5geode16ModMetadataLinksC1ERKS0_",
        android32: "_ZN5geode16ModMetadataLinksC1ERKS0_",
        android64: "_ZN5geode16ModMetadataLinksC1ERKS0_",
    }

    pub unsafe fn mod_metadata_links_dtor(this: *mut ModMetadataLinks) {
        win: "??1ModMetadataLinks@geode@@QEAA@XZ",
        mac_intel: "_ZN5geode16ModMetadataLinksD1Ev",
        mac_arm: "_ZN5geode16ModMetadataLinksD1Ev",
        ios: "_ZN5geode16ModMetadataLinksD1Ev",
        android32: "_ZN5geode16ModMetadataLinksD1Ev",
        android64: "_ZN5geode16ModMetadataLinksD1Ev",
    }

    pub unsafe fn mod_metadata_dependency_copy_ctor(this: *mut ModMetadataDependency, other: *const ModMetadataDependency) {
        win: "??0Dependency@ModMetadata@geode@@QEAA@AEBV012@@Z",
        mac_intel: "_ZN5geode11ModMetadata10DependencyC1ERKS1_",
        mac_arm: "_ZN5geode11ModMetadata10DependencyC1ERKS1_",
        ios: "_ZN5geode11ModMetadata10DependencyC1ERKS1_",
        android32: "_ZN5geode11ModMetadata10DependencyC1ERKS1_",
        android64: "_ZN5geode11ModMetadata10DependencyC1ERKS1_",
    }

    pub unsafe fn mod_metadata_dependency_dtor(this: *mut ModMetadataDependency) {
        win: "??1Dependency@ModMetadata@geode@@QEAA@XZ",
        mac_intel: "_ZN5geode11ModMetadata10DependencyD1Ev",
        mac_arm: "_ZN5geode11ModMetadata10DependencyD1Ev",
        ios: "_ZN5geode11ModMetadata10DependencyD1Ev",
        android32: "_ZN5geode11ModMetadata10DependencyD1Ev",
        android64: "_ZN5geode11ModMetadata10DependencyD1Ev",
    }

    pub unsafe fn mod_metadata_incompatibility_copy_ctor(this: *mut ModMetadataIncompatibility, other: *const ModMetadataIncompatibility) {
        win: "??0Incompatibility@ModMetadata@geode@@QEAA@AEBV012@@Z",
        mac_intel: "_ZN5geode11ModMetadata15IncompatibilityC1ERKS1_",
        mac_arm: "_ZN5geode11ModMetadata15IncompatibilityC1ERKS1_",
        ios: "_ZN5geode11ModMetadata15IncompatibilityC1ERKS1_",
        android32: "_ZN5geode11ModMetadata15IncompatibilityC1ERKS1_",
        android64: "_ZN5geode11ModMetadata15IncompatibilityC1ERKS1_",
    }

    pub unsafe fn mod_metadata_incompatibility_dtor(this: *mut ModMetadataIncompatibility) {
        win: "??1Incompatibility@ModMetadata@geode@@QEAA@XZ",
        mac_intel: "_ZN5geode11ModMetadata15IncompatibilityD1Ev",
        mac_arm: "_ZN5geode11ModMetadata15IncompatibilityD1Ev",
        ios: "_ZN5geode11ModMetadata15IncompatibilityD1Ev",
        android32: "_ZN5geode11ModMetadata15IncompatibilityD1Ev",
        android64: "_ZN5geode11ModMetadata15IncompatibilityD1Ev",
    }

    pub unsafe fn mod_metadata_issues_info_copy_ctor(this: *mut ModMetadataIssuesInfo, other: *const ModMetadataIssuesInfo) {
        win: "??0IssuesInfo@ModMetadata@geode@@QEAA@AEBV012@@Z",
        mac_intel: "_ZN5geode11ModMetadata10IssuesInfoC1ERKS1_",
        mac_arm: "_ZN5geode11ModMetadata10IssuesInfoC1ERKS1_",
        ios: "_ZN5geode11ModMetadata10IssuesInfoC1ERKS1_",
        android32: "_ZN5geode11ModMetadata10IssuesInfoC1ERKS1_",
        android64: "_ZN5geode11ModMetadata10IssuesInfoC1ERKS1_",
    }

    pub unsafe fn mod_metadata_issues_info_dtor(this: *mut ModMetadataIssuesInfo) {
        win: "??1IssuesInfo@ModMetadata@geode@@QEAA@XZ",
        mac_intel: "_ZN5geode11ModMetadata10IssuesInfoD1Ev",
        mac_arm: "_ZN5geode11ModMetadata10IssuesInfoD1Ev",
        ios: "_ZN5geode11ModMetadata10IssuesInfoD1Ev",
        android32: "_ZN5geode11ModMetadata10IssuesInfoD1Ev",
        android64: "_ZN5geode11ModMetadata10IssuesInfoD1Ev",
    }

    pub unsafe fn mod_metadata_create(json: *const MatJsonValue) -> sret ModMetadata {
        win: "?create@ModMetadata@geode@@SA?AV12@AEBVValue@matjson@@@Z",
        mac_intel: "_ZN5geode11ModMetadata6createERKN7matjson5ValueE",
        mac_arm: "_ZN5geode11ModMetadata6createERKN7matjson5ValueE",
        ios: "_ZN5geode11ModMetadata6createERKN7matjson5ValueE",
        android32: "_ZN5geode11ModMetadata6createERKN7matjson5ValueE",
        android64: "_ZN5geode11ModMetadata6createERKN7matjson5ValueE",
    }

    pub unsafe fn mod_metadata_create_from_geode_file(path: *const StlPath) -> sret ModMetadata {
        win: "?createFromGeodeFile@ModMetadata@geode@@SA?AV12@AEBVpath@filesystem@std@@@Z",
        mac_intel: "_ZN5geode11ModMetadata19createFromGeodeFileERKNSt3__14__fs10filesystem4pathE",
        mac_arm: "_ZN5geode11ModMetadata19createFromGeodeFileERKNSt3__14__fs10filesystem4pathE",
        ios: "_ZN5geode11ModMetadata19createFromGeodeFileERKNSt3__14__fs10filesystem4pathE",
        android32: "_ZN5geode11ModMetadata19createFromGeodeFileERKNSt6__ndk14__fs10filesystem4pathE",
        android64: "_ZN5geode11ModMetadata19createFromGeodeFileERKNSt6__ndk14__fs10filesystem4pathE",
    }

    pub unsafe fn mod_metadata_get_binary_name(metadata: *const ModMetadata) -> method_sret ZStringView {
        win: "?getBinaryName@ModMetadata@geode@@QEBA?AV?$BasicZStringView@D@2@XZ",
        mac_intel: "_ZNK5geode11ModMetadata13getBinaryNameEv",
        mac_arm: "_ZNK5geode11ModMetadata13getBinaryNameEv",
        ios: "_ZNK5geode11ModMetadata13getBinaryNameEv",
        android32: "_ZNK5geode11ModMetadata13getBinaryNameEv",
        android64: "_ZNK5geode11ModMetadata13getBinaryNameEv",
    }

    pub unsafe fn mod_metadata_get_id(metadata: *const ModMetadata) -> method_sret ZStringView {
        win: "?getID@ModMetadata@geode@@QEBA?AV?$BasicZStringView@D@2@XZ",
        mac_intel: "_ZNK5geode11ModMetadata5getIDEv",
        mac_arm: "_ZNK5geode11ModMetadata5getIDEv",
        ios: "_ZNK5geode11ModMetadata5getIDEv",
        android32: "_ZNK5geode11ModMetadata5getIDEv",
        android64: "_ZNK5geode11ModMetadata5getIDEv",
    }

    pub unsafe fn mod_metadata_get_name(metadata: *const ModMetadata) -> method_sret ZStringView {
        win: "?getName@ModMetadata@geode@@QEBA?AV?$BasicZStringView@D@2@XZ",
        mac_intel: "_ZNK5geode11ModMetadata7getNameEv",
        mac_arm: "_ZNK5geode11ModMetadata7getNameEv",
        ios: "_ZNK5geode11ModMetadata7getNameEv",
        android32: "_ZNK5geode11ModMetadata7getNameEv",
        android64: "_ZNK5geode11ModMetadata7getNameEv",
    }

    pub unsafe fn mod_metadata_get_version(metadata: *const ModMetadata) -> method_sret VersionInfo {
        win: "?getVersion@ModMetadata@geode@@QEBA?AVVersionInfo@2@XZ",
        mac_intel: "_ZNK5geode11ModMetadata10getVersionEv",
        mac_arm: "_ZNK5geode11ModMetadata10getVersionEv",
        ios: "_ZNK5geode11ModMetadata10getVersionEv",
        android32: "_ZNK5geode11ModMetadata10getVersionEv",
        android64: "_ZNK5geode11ModMetadata10getVersionEv",
    }

    pub unsafe fn mod_metadata_get_path(metadata: *const ModMetadata) -> *const StlPath {
        win: "?getPath@ModMetadata@geode@@QEBAAEBVpath@filesystem@std@@XZ",
        mac_intel: "_ZNK5geode11ModMetadata7getPathEv",
        mac_arm: "_ZNK5geode11ModMetadata7getPathEv",
        ios: "_ZNK5geode11ModMetadata7getPathEv",
        android32: "_ZNK5geode11ModMetadata7getPathEv",
        android64: "_ZNK5geode11ModMetadata7getPathEv",
    }

    pub unsafe fn mod_metadata_get_developers(metadata: *const ModMetadata) -> *const StlVector<StlString> {
        win: "?getDevelopers@ModMetadata@geode@@QEBAAEBV?$vector@V?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@V?$allocator@V?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@@2@@std@@XZ",
        mac_intel: "_ZNK5geode11ModMetadata13getDevelopersEv",
        mac_arm: "_ZNK5geode11ModMetadata13getDevelopersEv",
        ios: "_ZNK5geode11ModMetadata13getDevelopersEv",
        android32: "_ZNK5geode11ModMetadata13getDevelopersEv",
        android64: "_ZNK5geode11ModMetadata13getDevelopersEv",
    }

    pub unsafe fn mod_metadata_get_description(metadata: *const ModMetadata) -> *const StlOptional<StlString> {
        win: "?getDescription@ModMetadata@geode@@QEBAAEBV?$optional@V?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@@std@@XZ",
        mac_intel: "_ZNK5geode11ModMetadata14getDescriptionEv",
        mac_arm: "_ZNK5geode11ModMetadata14getDescriptionEv",
        ios: "_ZNK5geode11ModMetadata14getDescriptionEv",
        android32: "_ZNK5geode11ModMetadata14getDescriptionEv",
        android64: "_ZNK5geode11ModMetadata14getDescriptionEv",
    }

    pub unsafe fn mod_metadata_get_details(metadata: *const ModMetadata) -> *const StlOptional<StlString> {
        win: "?getDetails@ModMetadata@geode@@QEBAAEBV?$optional@V?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@@std@@XZ",
        mac_intel: "_ZNK5geode11ModMetadata10getDetailsEv",
        mac_arm: "_ZNK5geode11ModMetadata10getDetailsEv",
        ios: "_ZNK5geode11ModMetadata10getDetailsEv",
        android32: "_ZNK5geode11ModMetadata10getDetailsEv",
        android64: "_ZNK5geode11ModMetadata10getDetailsEv",
    }

    pub unsafe fn mod_metadata_get_links(metadata: *const ModMetadata) -> *const ModMetadataLinks {
        win: "?getLinks@ModMetadata@geode@@QEBAAEBVModMetadataLinks@2@XZ",
        mac_intel: "_ZNK5geode11ModMetadata8getLinksEv",
        mac_arm: "_ZNK5geode11ModMetadata8getLinksEv",
        ios: "_ZNK5geode11ModMetadata8getLinksEv",
        android32: "_ZNK5geode11ModMetadata8getLinksEv",
        android64: "_ZNK5geode11ModMetadata8getLinksEv",
    }

    pub unsafe fn mod_metadata_get_issues(metadata: *const ModMetadata) -> *const StlOptional<ModMetadataIssuesInfo> {
        win: "?getIssues@ModMetadata@geode@@QEBAAEBV?$optional@VIssuesInfo@ModMetadata@geode@@@std@@XZ",
        mac_intel: "_ZNK5geode11ModMetadata9getIssuesEv",
        mac_arm: "_ZNK5geode11ModMetadata9getIssuesEv",
        ios: "_ZNK5geode11ModMetadata9getIssuesEv",
        android32: "_ZNK5geode11ModMetadata9getIssuesEv",
        android64: "_ZNK5geode11ModMetadata9getIssuesEv",
    }

    pub unsafe fn mod_metadata_get_dependencies(metadata: *const ModMetadata) -> *const StlVector<ModMetadataDependency> {
        win: "?getDependencies@ModMetadata@geode@@QEBAAEBV?$vector@VDependency@ModMetadata@geode@@V?$allocator@VDependency@ModMetadata@geode@@@std@@@std@@XZ",
        mac_intel: "_ZNK5geode11ModMetadata15getDependenciesEv",
        mac_arm: "_ZNK5geode11ModMetadata15getDependenciesEv",
        ios: "_ZNK5geode11ModMetadata15getDependenciesEv",
        android32: "_ZNK5geode11ModMetadata15getDependenciesEv",
        android64: "_ZNK5geode11ModMetadata15getDependenciesEv",
    }

    pub unsafe fn mod_metadata_get_incompatibilities(metadata: *const ModMetadata) -> *const StlVector<ModMetadataIncompatibility> {
        win: "?getIncompatibilities@ModMetadata@geode@@QEBAAEBV?$vector@VIncompatibility@ModMetadata@geode@@V?$allocator@VIncompatibility@ModMetadata@geode@@@std@@@std@@XZ",
        mac_intel: "_ZNK5geode11ModMetadata20getIncompatibilitiesEv",
        mac_arm: "_ZNK5geode11ModMetadata20getIncompatibilitiesEv",
        ios: "_ZNK5geode11ModMetadata20getIncompatibilitiesEv",
        android32: "_ZNK5geode11ModMetadata20getIncompatibilitiesEv",
        android64: "_ZNK5geode11ModMetadata20getIncompatibilitiesEv",
    }

    pub unsafe fn mod_metadata_get_load_priority(metadata: *const ModMetadata) -> i32 {
        win: "?getLoadPriority@ModMetadata@geode@@QEBAHXZ",
        mac_intel: "_ZNK5geode11ModMetadata15getLoadPriorityEv",
        mac_arm: "_ZNK5geode11ModMetadata15getLoadPriorityEv",
        ios: "_ZNK5geode11ModMetadata15getLoadPriorityEv",
        android32: "_ZNK5geode11ModMetadata15getLoadPriorityEv",
        android64: "_ZNK5geode11ModMetadata15getLoadPriorityEv",
    }

    pub unsafe fn mod_metadata_get_raw_json(metadata: *const ModMetadata) -> method_sret MatJsonValue {
        win: "?getRawJSON@ModMetadata@geode@@QEBA?AVValue@matjson@@XZ",
        mac_intel: "_ZNK5geode11ModMetadata10getRawJSONEv",
        mac_arm: "_ZNK5geode11ModMetadata10getRawJSONEv",
        ios: "_ZNK5geode11ModMetadata10getRawJSONEv",
        android32: "_ZNK5geode11ModMetadata10getRawJSONEv",
        android64: "_ZNK5geode11ModMetadata10getRawJSONEv",
    }

    pub unsafe fn mod_metadata_to_json(metadata: *const ModMetadata) -> method_sret MatJsonValue {
        win: "?toJSON@ModMetadata@geode@@QEBA?AVValue@matjson@@XZ",
        mac_intel: "_ZNK5geode11ModMetadata6toJSONEv",
        mac_arm: "_ZNK5geode11ModMetadata6toJSONEv",
        ios: "_ZNK5geode11ModMetadata6toJSONEv",
        android32: "_ZNK5geode11ModMetadata6toJSONEv",
        android64: "_ZNK5geode11ModMetadata6toJSONEv",
    }

    pub unsafe fn mod_metadata_validate_id(id: StlStringView) -> bool {
        win: "?validateID@ModMetadata@geode@@SA_NV?$basic_string_view@DU?$char_traits@D@std@@@std@@@Z",
        mac_intel: "_ZN5geode11ModMetadata10validateIDENSt3__117basic_string_viewIcNS1_11char_traitsIcEEEE",
        mac_arm: "_ZN5geode11ModMetadata10validateIDENSt3__117basic_string_viewIcNS1_11char_traitsIcEEEE",
        ios: "_ZN5geode11ModMetadata10validateIDENSt3__117basic_string_viewIcNS1_11char_traitsIcEEEE",
        android32: "_ZN5geode11ModMetadata10validateIDENSt6__ndk117basic_string_viewIcNS1_11char_traitsIcEEEE",
        android64: "_ZN5geode11ModMetadata10validateIDENSt6__ndk117basic_string_viewIcNS1_11char_traitsIcEEEE",
    }

    pub unsafe fn mod_metadata_format_developer_display_string(developers: *const StlVector<StlString>) -> sret StlString {
        win: "?formatDeveloperDisplayString@ModMetadata@geode@@SA?AV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@AEBV?$vector@V?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@V?$allocator@V?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@@2@@4@@Z",
        mac_intel: "_ZN5geode11ModMetadata28formatDeveloperDisplayStringERKNSt3__16vectorINS1_12basic_stringIcNS1_11char_traitsIcEENS1_9allocatorIcEEEENS6_IS8_EEEE",
        mac_arm: "_ZN5geode11ModMetadata28formatDeveloperDisplayStringERKNSt3__16vectorINS1_12basic_stringIcNS1_11char_traitsIcEENS1_9allocatorIcEEEENS6_IS8_EEEE",
        ios: "_ZN5geode11ModMetadata28formatDeveloperDisplayStringERKNSt3__16vectorINS1_12basic_stringIcNS1_11char_traitsIcEENS1_9allocatorIcEEEENS6_IS8_EEEE",
        android32: "_ZN5geode11ModMetadata28formatDeveloperDisplayStringERKNSt6__ndk16vectorINS1_12basic_stringIcNS1_11char_traitsIcEENS1_9allocatorIcEEEENS6_IS8_EEEE",
        android64: "_ZN5geode11ModMetadata28formatDeveloperDisplayStringERKNSt6__ndk16vectorINS1_12basic_stringIcNS1_11char_traitsIcEENS1_9allocatorIcEEEENS6_IS8_EEEE",
    }

    pub unsafe fn mod_metadata_links_get_homepage_url(links: *const ModMetadataLinks) -> method_sret StlOptional<StlString> {
        win: "?getHomepageURL@ModMetadataLinks@geode@@QEBA?AV?$optional@V?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@@std@@XZ",
        mac_intel: "_ZNK5geode16ModMetadataLinks14getHomepageURLEv",
        mac_arm: "_ZNK5geode16ModMetadataLinks14getHomepageURLEv",
        ios: "_ZNK5geode16ModMetadataLinks14getHomepageURLEv",
        android32: "_ZNK5geode16ModMetadataLinks14getHomepageURLEv",
        android64: "_ZNK5geode16ModMetadataLinks14getHomepageURLEv",
    }

    pub unsafe fn mod_metadata_links_get_source_url(links: *const ModMetadataLinks) -> method_sret StlOptional<StlString> {
        win: "?getSourceURL@ModMetadataLinks@geode@@QEBA?AV?$optional@V?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@@std@@XZ",
        mac_intel: "_ZNK5geode16ModMetadataLinks12getSourceURLEv",
        mac_arm: "_ZNK5geode16ModMetadataLinks12getSourceURLEv",
        ios: "_ZNK5geode16ModMetadataLinks12getSourceURLEv",
        android32: "_ZNK5geode16ModMetadataLinks12getSourceURLEv",
        android64: "_ZNK5geode16ModMetadataLinks12getSourceURLEv",
    }

    pub unsafe fn mod_metadata_links_get_community_url(links: *const ModMetadataLinks) -> method_sret StlOptional<StlString> {
        win: "?getCommunityURL@ModMetadataLinks@geode@@QEBA?AV?$optional@V?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@@std@@XZ",
        mac_intel: "_ZNK5geode16ModMetadataLinks15getCommunityURLEv",
        mac_arm: "_ZNK5geode16ModMetadataLinks15getCommunityURLEv",
        ios: "_ZNK5geode16ModMetadataLinks15getCommunityURLEv",
        android32: "_ZNK5geode16ModMetadataLinks15getCommunityURLEv",
        android64: "_ZNK5geode16ModMetadataLinks15getCommunityURLEv",
    }

    pub unsafe fn mod_metadata_issues_info_get_info(info: *const ModMetadataIssuesInfo) -> *const StlString {
        win: "?getInfo@IssuesInfo@ModMetadata@geode@@QEBAAEBV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@XZ",
        mac_intel: "_ZNK5geode11ModMetadata10IssuesInfo7getInfoEv",
        mac_arm: "_ZNK5geode11ModMetadata10IssuesInfo7getInfoEv",
        ios: "_ZNK5geode11ModMetadata10IssuesInfo7getInfoEv",
        android32: "_ZNK5geode11ModMetadata10IssuesInfo7getInfoEv",
        android64: "_ZNK5geode11ModMetadata10IssuesInfo7getInfoEv",
    }

    pub unsafe fn mod_metadata_issues_info_get_url(info: *const ModMetadataIssuesInfo) -> *const StlOptional<StlString> {
        win: "?getURL@IssuesInfo@ModMetadata@geode@@QEBAAEBV?$optional@V?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@@std@@XZ",
        mac_intel: "_ZNK5geode11ModMetadata10IssuesInfo6getURLEv",
        mac_arm: "_ZNK5geode11ModMetadata10IssuesInfo6getURLEv",
        ios: "_ZNK5geode11ModMetadata10IssuesInfo6getURLEv",
        android32: "_ZNK5geode11ModMetadata10IssuesInfo6getURLEv",
        android64: "_ZNK5geode11ModMetadata10IssuesInfo6getURLEv",
    }

    pub unsafe fn mod_metadata_dependency_get_id(dependency: *const ModMetadataDependency) -> *const StlString {
        win: "?getID@Dependency@ModMetadata@geode@@QEBAAEBV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@XZ",
        mac_intel: "_ZNK5geode11ModMetadata10Dependency5getIDEv",
        mac_arm: "_ZNK5geode11ModMetadata10Dependency5getIDEv",
        ios: "_ZNK5geode11ModMetadata10Dependency5getIDEv",
        android32: "_ZNK5geode11ModMetadata10Dependency5getIDEv",
        android64: "_ZNK5geode11ModMetadata10Dependency5getIDEv",
    }

    pub unsafe fn mod_metadata_dependency_get_version(dependency: *const ModMetadataDependency) -> *const ComparableVersionInfo {
        win: "?getVersion@Dependency@ModMetadata@geode@@QEBAAEBVComparableVersionInfo@3@XZ",
        mac_intel: "_ZNK5geode11ModMetadata10Dependency10getVersionEv",
        mac_arm: "_ZNK5geode11ModMetadata10Dependency10getVersionEv",
        ios: "_ZNK5geode11ModMetadata10Dependency10getVersionEv",
        android32: "_ZNK5geode11ModMetadata10Dependency10getVersionEv",
        android64: "_ZNK5geode11ModMetadata10Dependency10getVersionEv",
    }

    pub unsafe fn mod_metadata_dependency_is_required(dependency: *const ModMetadataDependency) -> bool {
        win: "?isRequired@Dependency@ModMetadata@geode@@QEBA_NXZ",
        mac_intel: "_ZNK5geode11ModMetadata10Dependency10isRequiredEv",
        mac_arm: "_ZNK5geode11ModMetadata10Dependency10isRequiredEv",
        ios: "_ZNK5geode11ModMetadata10Dependency10isRequiredEv",
        android32: "_ZNK5geode11ModMetadata10Dependency10isRequiredEv",
        android64: "_ZNK5geode11ModMetadata10Dependency10isRequiredEv",
    }

    pub unsafe fn mod_metadata_dependency_get_mod(dependency: *const ModMetadataDependency) -> *mut c_void {
        win: "?getMod@Dependency@ModMetadata@geode@@QEBAPEAVMod@3@XZ",
        mac_intel: "_ZNK5geode11ModMetadata10Dependency6getModEv",
        mac_arm: "_ZNK5geode11ModMetadata10Dependency6getModEv",
        ios: "_ZNK5geode11ModMetadata10Dependency6getModEv",
        android32: "_ZNK5geode11ModMetadata10Dependency6getModEv",
        android64: "_ZNK5geode11ModMetadata10Dependency6getModEv",
    }

    pub unsafe fn mod_metadata_dependency_get_settings(dependency: *const ModMetadataDependency) -> *const MatJsonValue {
        win: "?getSettings@Dependency@ModMetadata@geode@@QEBAAEBVValue@matjson@@XZ",
        mac_intel: "_ZNK5geode11ModMetadata10Dependency11getSettingsEv",
        mac_arm: "_ZNK5geode11ModMetadata10Dependency11getSettingsEv",
        ios: "_ZNK5geode11ModMetadata10Dependency11getSettingsEv",
        android32: "_ZNK5geode11ModMetadata10Dependency11getSettingsEv",
        android64: "_ZNK5geode11ModMetadata10Dependency11getSettingsEv",
    }

    pub unsafe fn mod_metadata_dependency_is_resolved(dependency: *const ModMetadataDependency) -> bool {
        win: "?isResolved@Dependency@ModMetadata@geode@@QEBA_NXZ",
        mac_intel: "_ZNK5geode11ModMetadata10Dependency10isResolvedEv",
        mac_arm: "_ZNK5geode11ModMetadata10Dependency10isResolvedEv",
        ios: "_ZNK5geode11ModMetadata10Dependency10isResolvedEv",
        android32: "_ZNK5geode11ModMetadata10Dependency10isResolvedEv",
        android64: "_ZNK5geode11ModMetadata10Dependency10isResolvedEv",
    }

    pub unsafe fn mod_metadata_incompatibility_get_id(incompatibility: *const ModMetadataIncompatibility) -> *const StlString {
        win: "?getID@Incompatibility@ModMetadata@geode@@QEBAAEBV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@XZ",
        mac_intel: "_ZNK5geode11ModMetadata15Incompatibility5getIDEv",
        mac_arm: "_ZNK5geode11ModMetadata15Incompatibility5getIDEv",
        ios: "_ZNK5geode11ModMetadata15Incompatibility5getIDEv",
        android32: "_ZNK5geode11ModMetadata15Incompatibility5getIDEv",
        android64: "_ZNK5geode11ModMetadata15Incompatibility5getIDEv",
    }

    pub unsafe fn mod_metadata_incompatibility_get_version(incompatibility: *const ModMetadataIncompatibility) -> *const ComparableVersionInfo {
        win: "?getVersion@Incompatibility@ModMetadata@geode@@QEBAAEBVComparableVersionInfo@3@XZ",
        mac_intel: "_ZNK5geode11ModMetadata15Incompatibility10getVersionEv",
        mac_arm: "_ZNK5geode11ModMetadata15Incompatibility10getVersionEv",
        ios: "_ZNK5geode11ModMetadata15Incompatibility10getVersionEv",
        android32: "_ZNK5geode11ModMetadata15Incompatibility10getVersionEv",
        android64: "_ZNK5geode11ModMetadata15Incompatibility10getVersionEv",
    }

    pub unsafe fn mod_metadata_incompatibility_is_breaking(incompatibility: *const ModMetadataIncompatibility) -> bool {
        win: "?isBreaking@Incompatibility@ModMetadata@geode@@QEBA_NXZ",
        mac_intel: "_ZNK5geode11ModMetadata15Incompatibility10isBreakingEv",
        mac_arm: "_ZNK5geode11ModMetadata15Incompatibility10isBreakingEv",
        ios: "_ZNK5geode11ModMetadata15Incompatibility10isBreakingEv",
        android32: "_ZNK5geode11ModMetadata15Incompatibility10isBreakingEv",
        android64: "_ZNK5geode11ModMetadata15Incompatibility10isBreakingEv",
    }

    pub unsafe fn mod_metadata_incompatibility_get_mod(incompatibility: *const ModMetadataIncompatibility) -> *mut c_void {
        win: "?getMod@Incompatibility@ModMetadata@geode@@QEBAPEAVMod@3@XZ",
        mac_intel: "_ZNK5geode11ModMetadata15Incompatibility6getModEv",
        mac_arm: "_ZNK5geode11ModMetadata15Incompatibility6getModEv",
        ios: "_ZNK5geode11ModMetadata15Incompatibility6getModEv",
        android32: "_ZNK5geode11ModMetadata15Incompatibility6getModEv",
        android64: "_ZNK5geode11ModMetadata15Incompatibility6getModEv",
    }

    pub unsafe fn mod_metadata_incompatibility_is_resolved(incompatibility: *const ModMetadataIncompatibility) -> bool {
        win: "?isResolved@Incompatibility@ModMetadata@geode@@QEBA_NXZ",
        mac_intel: "_ZNK5geode11ModMetadata15Incompatibility10isResolvedEv",
        mac_arm: "_ZNK5geode11ModMetadata15Incompatibility10isResolvedEv",
        ios: "_ZNK5geode11ModMetadata15Incompatibility10isResolvedEv",
        android32: "_ZNK5geode11ModMetadata15Incompatibility10isResolvedEv",
        android64: "_ZNK5geode11ModMetadata15Incompatibility10isResolvedEv",
    }

}

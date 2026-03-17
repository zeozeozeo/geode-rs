use crate::geode_bind;
use crate::loader::{ByteSpan, GeodeResult};
use crate::stl::{StlOptional, StlSharedPtr, StlString, StlStringView, StlVector, ZStringView};
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

}

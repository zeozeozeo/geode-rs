#pragma once

#include "cplatform.h"
#include <string>
#include <functional>
#include <memory>

#define GEODE_HIDDEN
#define GEODE_INLINE inline
#define GEODE_VIRTUAL_CONSTEXPR
#define GEODE_NOINLINE

#ifdef GEODE_IS_WINDOWS
    #define GEODE_DLL
    #define GEODE_API extern "C"
    #define GEODE_EXPORT
    #define GEODE_NO_UNIQUE_ADDRESS
#endif

#define GEODE_PRETTY_FUNCTION ""
#define GEODE_WRAPPER_STR(...) #__VA_ARGS__
#define GEODE_STR(...) GEODE_WRAPPER_STR(__VA_ARGS__)

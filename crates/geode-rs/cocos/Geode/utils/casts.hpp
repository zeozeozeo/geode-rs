#pragma once

#include <utility>
#include <typeinfo>

namespace geode {
    namespace cast {
        template<typename To, typename From>
        To typeinfo_cast(From&& f) { 
            return dynamic_cast<To>(std::forward<From>(f)); 
        }
        
        template<typename To, typename From>
        To unionCast(From f) { return reinterpret_cast<To>(f); }
        
        template<typename To, typename From>
        To bitCast(From f) { return *reinterpret_cast<To*>(&f); }
    }
}

#pragma once

#include <string>
#include <functional>

namespace geode {
    template<typename EventT, typename FuncSig = void(), typename... Args>
    class Event {
    public:
        void* m_sender;
    };
    
    template<typename EventT, typename FuncSig = void(), typename... Args>
    class EventListener {
    public:
        void* m_callback;
    };
}

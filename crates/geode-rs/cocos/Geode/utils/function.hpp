#pragma once

#include <functional>

namespace geode {
    template<typename T>
    using Function = std::function<T>;
}

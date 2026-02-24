// Stubs for some geode apis...

#ifndef GEODE_DEFAULT_INCLUDE_HPP
#define GEODE_DEFAULT_INCLUDE_HPP

#include <cstdint>
#include <cstddef>
#include <cstring>
#include <string>

#define GEODE_FRIEND_MODIFY
#define GEODE_DLL

#define GEODE_CUSTOM_CONSTRUCTOR_BEGIN(Class_) \
    Class_(geode::ZeroConstructorType, void*) {} \
    Class_(geode::ZeroConstructorType, size_t) {} \
    Class_(geode::ZeroConstructorType) {} \
    Class_(geode::CutoffConstructorType, size_t) {} \
    Class_(geode::CutoffConstructorType, void*) {}

#define GEODE_CUSTOM_CONSTRUCTOR_COCOS(Class_, Base_) \
    Class_(geode::ZeroConstructorType, size_t) : Base_(geode::ZeroConstructor, sizeof(Class_)) {} \
    Class_(geode::ZeroConstructorType) : Base_(geode::ZeroConstructor, sizeof(Class_)) {} \
    Class_(geode::CutoffConstructorType, size_t) : Base_(geode::CutoffConstructor, sizeof(Class_)) {}

#define GEODE_CUSTOM_CONSTRUCTOR_GD(Class_, Base_) \
    Class_(geode::ZeroConstructorType, size_t) : Base_(geode::ZeroConstructor, sizeof(Class_)) {} \
    Class_(geode::ZeroConstructorType) : Base_(geode::ZeroConstructor, sizeof(Class_)) {} \
    Class_(geode::CutoffConstructorType, size_t) : Base_(geode::CutoffConstructor, sizeof(Class_)) {}

#define GEODE_ZERO_CONSTRUCTOR_BEGIN(Class_) \
    Class_(geode::ZeroConstructorType, void*) {} \
    Class_(geode::ZeroConstructorType, size_t) {} \
    Class_(geode::ZeroConstructorType) {}

#define GEODE_ZERO_CONSTRUCTOR(Class_, Base_) \
    Class_(geode::ZeroConstructorType, size_t) : Base_(geode::ZeroConstructor, sizeof(Class_)) {} \
    Class_(geode::ZeroConstructorType) : Base_(geode::ZeroConstructor, sizeof(Class_)) {}

#define GEODE_CUTOFF_CONSTRUCTOR_BEGIN(Class_) \
    Class_(geode::CutoffConstructorType, size_t) {} \
    Class_(geode::CutoffConstructorType, void*) {}

#define GEODE_CUTOFF_CONSTRUCTOR_COCOS(Class_, Base_) \
    Class_(geode::CutoffConstructorType, size_t) : Base_(geode::CutoffConstructor, sizeof(Class_)) {}

#define GEODE_CUTOFF_CONSTRUCTOR_GD(Class_, Base_) \
    Class_(geode::CutoffConstructorType, size_t) : Base_(geode::CutoffConstructor, sizeof(Class_)) {}

#define GEODE_CUTOFF_CONSTRUCTOR_CUTOFF(Class_, Base_) \
    Class_(geode::CutoffConstructorType, size_t) : Base_(geode::CutoffConstructor, sizeof(Class_)) {}

#define GEODE_NONINHERITED_MEMBERS
#define GEODE_FILL_CONSTRUCTOR(Class_, Offset_) \

struct INPUT {
    int type;
    union {
        int mi;
        int ki;
        int hi;
    } data;
};

namespace gd {
    template<typename T>
    class vector {
        T* _data;
        size_t _size;
        size_t _capacity;
    public:
        T* data() { return _data; }
        size_t size() const { return _size; }
    };

    template<typename K, typename V>
    class map {};

    template<typename K, typename V>
    class unordered_map {};

    template<typename K>
    class set {
    public:
        class iterator {};
    };

    template<typename K>
    class unordered_set {};

    class string {
        char* _data;
        size_t _size;
        size_t _capacity;
    public:
        string() : _data(nullptr), _size(0), _capacity(0) {}
        string(const char* s) : _data(const_cast<char*>(s)), _size(0), _capacity(0) {}
        string(const char* s, size_t len) : _data(const_cast<char*>(s)), _size(len), _capacity(len) {}
        string(const string& o) : _data(o._data), _size(o._size), _capacity(o._capacity) {}
        string(const std::string& s) : _data(const_cast<char*>(s.c_str())), _size(s.size()), _capacity(s.capacity()) {}
        ~string() {}

        string& operator=(const string& o) { _data = o._data; _size = o._size; _capacity = o._capacity; return *this; }
        string& operator=(const char* s) { _data = const_cast<char*>(s); return *this; }
        string& operator=(char* s) { _data = s; return *this; }

        const char* c_str() const { return _data ? _data : ""; }
        size_t size() const { return _size; }
        bool empty() const { return _size == 0; }
        char& operator[](size_t i) { return _data[i]; }
        const char& operator[](size_t i) const { return _data[i]; }
    };

    template<typename T1, typename T2>
    struct pair {
        T1 first;
        T2 second;
    };
}

namespace geode {
    struct ZeroConstructorType {};
    static constexpr auto ZeroConstructor = ZeroConstructorType();

    struct CutoffConstructorType {};
    static constexpr auto CutoffConstructor = CutoffConstructorType();

    template<typename T>
    struct SeedValueRSV {
        T _value;
    };

    namespace comm {
        template<typename T>
        class Vec {
        public:
            T* m_data;
            size_t m_size;
        };

        template<typename K, typename V>
        class Map {};

        class ListenerHandle {
        public:
            void* m_handle;
        };
    }

    namespace cocos {
        template<typename T, bool R = true>
        class CCArrayExt {
        public:
            void* m_array;
        };

        template<typename K, typename V, bool R = true>
        class CCDictionaryExt {
        public:
            void* m_dict;
        };
    }
}

#endif

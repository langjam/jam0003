#pragma once

#include <optional>
#include <cstddef>
#include <cassert>

template<typename T>
class ErrorOr {
public:
    ErrorOr()
        : m_is_error(true) { }
    ErrorOr(T value) 
        : m_is_error(false), m_value(value) { }

    ~ErrorOr() { }

    bool is_error() { return m_is_error; }

    T value() {
        assert(m_value.has_value());
        return m_value.value();
    }

private:
    bool m_is_error;
    std::optional<T> m_value;
};

template<typename T>
class ErrorOr<void> {
public:
    ErrorOr(bool is_value)
        : m_is_error(!is_value) { }

    ~ErrorOr() { }

    bool is_error() { return m_is_error; }

private:
    bool m_is_error;

    // Just so don't get screamed at
    T value() { }
};

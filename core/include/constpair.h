#pragma once
#include <iostream>

template <typename T1, typename T2> class ConstPair
{
public:
    constexpr ConstPair() : first(), second() {}
    constexpr ConstPair(ConstPair<T1, T2> const& pair) : first(pair.first), second(pair.second) {}
    constexpr ConstPair(T1 const& first, T2 const& second) : first(first), second(second) {}
    constexpr ConstPair(T1 const& first) : first(first), second() {}

    constexpr ConstPair<T1, T2>& operator=(ConstPair<T1, T2> const& other)
    {
        first = other.first;
        second = other.second;
        return *this;
    }

    T1 first;
    T2 second;
};

template <typename T1, typename T2>
std::ostream& operator<<(std::ostream& stream, ConstPair<T1, T2> const& pair)
{
    stream << "[Pair]\nFirst:\n" << pair.first << "\nSecond:\n" << pair.second;
    return stream;
}

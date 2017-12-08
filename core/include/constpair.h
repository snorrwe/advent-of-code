#pragma once

template <typename T1, typename T2> class ConstPair
{
public:
    constexpr ConstPair() : first(), second() {}
    constexpr ConstPair(ConstPair<T1, T2> const& pair) : first(pair.first), second(pair.second) {}
    constexpr ConstPair(T1 const& first, T2 const& second) : first(first), second(second) {}
    constexpr ConstPair(T1 const& first) : first(first) {}

    constexpr ConstPair<T1, T2>& operator=(ConstPair<T1, T2> const& other)
    {
        first = other.first;
        second = other.second;
        return *this;
    }

    T1 first;
    T2 second;
};

#pragma once
#include <iostream>

struct Word
{
    constexpr Word() : begin(), end() {}
    constexpr Word(const char* begin, const char* end) : begin(begin), end(end) {}
    constexpr Word(Word const& w) : begin(w.begin), end(w.end) {}

    constexpr Word& operator=(Word const& w)
    {
        begin = w.begin;
        end = w.end;
        return *this;
    }
    constexpr bool operator==(Word const& w)
    {
        auto i1 = begin;
        auto i2 = w.begin;
        for (; i1 != end && i2 != w.end; ++i1, ++i2)
        {
            if (*i1 != *i2) return false;
        }
        return i1 == end && i2 == w.end;
    }

    const char* begin;
    const char* end;
};

std::ostream& operator<<(std::ostream& stream, Word const& word)
{
    for (auto i = word.begin; i != word.end; ++i)
    {
        stream << *i;
    }
    return stream;
}

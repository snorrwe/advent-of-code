#pragma once
#include "constpair.h"
#include <array>
#include <iostream>

constexpr int constPow(int base, unsigned power)
{
    int result = 1;
    for (int i = 0; i < power; ++i)
    {
        result *= base;
    }
    return result;
}

struct Word
{
    template <size_t MaxSize> using SplitResult = ConstPair<std::array<Word, MaxSize>, size_t>;
    template <size_t MaxSize = 10>
    constexpr static SplitResult<MaxSize> split(const char* begin, char sep = ' ');
    constexpr static size_t size(Word const& word);
    friend std::ostream& operator<<(std::ostream& stream, Word const& word);
    friend constexpr bool operator==(Word const& w1, Word const& w2);

    constexpr Word() : begin(nullptr), end(nullptr) {}
    constexpr Word(const char* begin, const char* end) : begin(begin), end(end) {}
    constexpr Word(Word const& w) : begin(w.begin), end(w.end) {}

    constexpr Word& operator=(Word const& w)
    {
        begin = w.begin;
        end = w.end;
        return *this;
    }

    constexpr int toInt() const
    {
        if (begin == end) return 0;
        size_t cnt = 0;
        auto i = end;
        int result = 0;
        while (i-- != begin && *i >= '0' && *i <= '9')
        {
            result += (*i - '0') * constPow(10, cnt++);
        }
        if (*begin == '-')
        {
            result *= -1;
        }
        return result;
    }

    constexpr bool operator!=(Word const& w) const { return !(*this == w); }

    const char* begin;
    const char* end;
};

constexpr size_t Word::size(Word const& word)
{
    // TODO: fix clang not recognising this as constexpr
    size_t result = 0;
    for (auto i = word.begin; i != word.end; ++i)
    {
        ++result;
    }
    return result;
}

constexpr bool operator==(Word const& w1, Word const& w2)
{
    // if (Word::size(w1) != Word::size(w2)) return false; //TODO: reenable once size() is constexpr
    auto i1 = w1.begin;
    auto i2 = w2.begin;
    while (i1 != w1.end)
    {
        if (*i1 != *i2) return false;
        ++i1;
        ++i2;
    }
    return true;
}

constexpr bool operator==(Word const& w1, const char* word)
{
    auto i1 = w1.begin;
    auto i2 = word;
    while (i1 != w1.end)
    {
        if (*i1 != *i2) return false;
        if (*i2 == '\0') return false;
        ++i1;
        ++i2;
    }
    return *i2 == '\0';
}

constexpr bool operator==(const char* word, Word const& w1) { return w1 == word; }

template <size_t size>
constexpr bool operator==(std::array<char, size> const& chars, const char* word)
{
    auto wit = word;
    for (int i = 0; i < size && *wit != '\0'; ++i)
    {
        if (chars[i] != *wit) return false;
        ++wit;
    }
    return *wit == '\0';
}

template <size_t N> std::ostream& operator<<(std::ostream& stream, std::array<char, N> arr)
{
    stream << "Char array: [";
    for (int i = 0; i < N; ++i)
    {
        stream << arr[i];
    }
    stream << "]";
    return stream;
}

template <size_t MaxSize>
constexpr ConstPair<std::array<Word, MaxSize>, size_t> Word::split(const char* begin, char sep)
{
    auto result = std::array<Word, MaxSize>();
    size_t size = 0;
    bool inWord = *begin != sep;
    auto end = begin;
    while (*end != '\0' && *end != '\n')
    {
        if (inWord && *end == sep)
        {
            result[size++] = Word(begin, end);
            inWord = false;
        }
        else if (!inWord && *end != sep)
        {
            inWord = true;
            begin = end;
        }
        ++end;
    }
    if (inWord)
    {
        result[size++] = Word(begin, end);
    }
    return ConstPair(result, size);
}

std::ostream& operator<<(std::ostream& stream, Word const& word)
{
    for (auto i = word.begin; i != word.end; ++i)
    {
        stream << *i;
    }
    return stream;
}

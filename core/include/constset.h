#pragma once
#include "constpair.h"
#include <iostream>

namespace ConstSet
{
typedef ConstPair<TSet, size_t> TAddResult;
typedef ConstPair<bool, TNode> TFindResult;
typedef ConstPair<bool, size_t> TFindIndexResult;

constexpr TFindResult find(TSet const& values, Vector2 const& value, size_t size)
{
    for (int i = 0; i < size; ++i)
    {
        if (value == values[i].first) return TFindResult(true, values[i]);
    }
    return TFindResult(false);
}

constexpr TFindIndexResult findIndex(TSet const& values, Vector2 const& value, size_t size)
{
    for (int i = 0; i < size; ++i)
    {
        if (value == values[i].first) return TFindIndexResult(true, i);
    }
    return TFindIndexResult(false, -1);
}

constexpr TAddResult add(TSet const& values, TNode const& value, size_t size)
{
    auto fresult = findIndex(values, value.first, size);
    auto result = TSet(values);
    if (!fresult.first)
    {
        result[size++] = value;
    }
    return TAddResult(result, size);
}
};

#pragma once
#include "constpair.h"
#include "vector2.h"
#include <iostream>

namespace ConstSet
{
typedef ConstPair<bool, size_t> TFindIndexResult;
template <typename T> using TFindResult = ConstPair<bool, T>;
template <typename T> using TAddResult = ConstPair<T, size_t>;

template <typename TSet>
constexpr TFindIndexResult findIndex(TSet const& values, Vector2 const& value, size_t size)
{
    for (int i = 0; i < size; ++i)
    {
        if (value == values[i].first) return TFindIndexResult(true, i);
    }
    return TFindIndexResult(false, -1);
}

template <typename TSet, typename TResult>
constexpr TFindResult<TResult> find(TSet const& values, Vector2 const& value, size_t size)
{
    auto indexResult = findIndex<TSet>(values, value, size);
    if (indexResult.first) return TFindResult<TResult>(true, values[indexResult.second]);
    return TFindResult<TResult>(false);
}

template <typename TSet, typename TNode>
constexpr TAddResult<TSet> add(TSet const& values, TNode const& value, size_t size)
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

#pragma once
#include "constpair.h"
#include <array>
#include <iostream>

template <typename TKey, typename TValue, size_t MaxSize = 1000> class ConstDict
{
public:
    using Node = ConstPair<TKey, TValue>;
    using Container = std::array<Node, MaxSize>;
    using ContainerInfo = ConstPair<Container, size_t>;

    constexpr static Container create() { return Container(); }

    constexpr static ConstPair<bool, size_t> findIndex(Container const& container, size_t size,
                                                       TKey const& key)
    {
        for (int i = 0; i < size; ++i)
        {
            auto& node = container[i];
            if (node.first == key) return ConstPair<bool, size_t>(true, i);
        }
        return ConstPair<bool, size_t>(false, -1);
    }

    constexpr static ConstPair<bool, Node> find(Container const& container, size_t size,
                                                TKey const& key)
    {
        auto result = findIndex(container, size, key);
        if (result.first) return ConstPair<bool, Node>(true, container[result.second]);
        return ConstPair<bool, Node>(false);
    }

    constexpr static TValue getValue(Container const& container, size_t size, TKey const& key,
                                     TValue const& defaultValue)
    {
        auto existing = find(container, size, key);
        if (existing.first)
        {
            return existing.second.second;
        }
        return defaultValue;
    }

    constexpr static ContainerInfo remove(Container const& container, size_t size, TKey const& key)
    {
        auto result = Container();
        size_t newSize = 0;
        for (int i = 0; i < size; ++i)
        {
            if (container[i].first != key) result[newSize++] = container[i];
        }
        return ConstPair(result, newSize);
    }

    constexpr static ContainerInfo mutate(Container const& container, size_t size, TKey const& key,
                                          TValue const& value)
    {
        auto result = Container(container);
        auto index = findIndex(container, size, key);
        if (index.first)
        {
            result[index.second] = Node(key, value);
        }
        return result;
    }

    constexpr static ContainerInfo add(Container const& container, size_t size, TKey const& key,
                                       TValue const& value)
    {
        auto result = Container(container);
        result[size++] = Node(key, value);
        return ConstPair(result, size);
    }
};

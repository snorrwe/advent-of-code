#pragma once
#include <iostream>

class Vector2
{
public:
    constexpr Vector2() : x(0), y(0) {}
    constexpr Vector2(size_t x, size_t y) : x(x), y(y) {}
    constexpr Vector2(Vector2&& v) : x(v.x), y(v.y) {}
    constexpr Vector2(Vector2 const& v) : x(v.x), y(v.y) {}
    constexpr Vector2& operator=(Vector2 const&) = default;

    constexpr Vector2 turnLeft() { return Vector2(-y, x); }
    constexpr size_t distance(Vector2 const& v)
    {
        auto x = this->x - v.x;
        auto y = this->y - v.y;
        if (x < 0) x = -x;
        if (y < 0) y = -y;
        return x + y;
    }

    int x;
    int y;
};

constexpr Vector2 operator+(Vector2 const& lhs, Vector2 const& rhs)
{
    return Vector2(lhs.x + rhs.x, lhs.y + rhs.y);
}

constexpr Vector2 operator-(Vector2 const& lhs, Vector2 const& rhs)
{
    return Vector2(lhs.x - rhs.x, lhs.y - rhs.y);
}

constexpr bool operator==(Vector2 const& lhs, Vector2 const& rhs)
{
    return lhs.x == rhs.x && lhs.y == rhs.y;
}

std::ostream& operator<<(std::ostream& stream, Vector2 const& vector)
{
    stream << "[Vector] (" << vector.x << ";" << vector.y << ")";
    return stream;
}

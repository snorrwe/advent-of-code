#pragma once
#include "day10input.hpp"
#include "minomaly/minomaly.hpp"
#include "minomaly/system/input_system.hpp"
#include "minomaly/system/render_system.hpp"
#include <iostream>
#include <limits>
#include <random>

constexpr auto STAR_SIZE = 10;

struct Velocity : public mino::Point
{
    mino::Point value;
    using mino::Point::Point;
    using mino::Point::operator+;
    using mino::Point::operator+=;
};

class MapSystem final : public mino::ISystem
{
    mino::Minomaly& engine;
    mino::RenderSystem* render_system = nullptr;
    mino::InputSystem* input_system = nullptr;
    mino::Manager<mino::RenderComponent>* renders = nullptr;
    mino::Manager<mino::PositionComponent>* positions = nullptr;
    mino::Manager<Velocity>* velocities = nullptr;
    mino::Logger* logger = nullptr;

    bool active = true;
    size_t count = 0;

public:
    explicit MapSystem(mino::Minomaly& engine)
        : engine(engine), logger(engine.get_log_manager()->get_logger("map_system"))
    {
    }
    ~MapSystem() = default;

    void start() override
    {
        logger->info("MapSystem is starting");

        render_system = engine.get_system<mino::RenderSystem>();
        input_system = engine.get_system<mino::InputSystem>();

        renders = engine.get_or_create_manager<mino::Manager<mino::RenderComponent>>();
        positions = engine.get_or_create_manager<mino::Manager<mino::PositionComponent>>();
        velocities = engine.get_or_create_manager<mino::Manager<Velocity>>();

        assert(render_system);
        assert(input_system);
        assert(renders);
        assert(positions);
        assert(velocities);

        init();
        for (int i = 0; i < 10000; ++i)
        {
            update_cells();
        }
        logger->info("MapSystem started successfully");
    }

    void update() override
    {
        for (auto& event : input_system->get_events())
        {
            if (event.type == SDL_KEYDOWN)
            {
                if (event.key.keysym.sym == SDLK_SPACE)
                {
                    active = !active;
                }
                else if (event.key.keysym.sym == SDLK_e && !active)
                {
                    update_cells();
                    logger->info("Tick {}", count);
                }
                else if (event.key.keysym.sym == SDLK_q)
                {
                    logger->info("Exiting");
                    engine.stop();
                }
            }
        }
        if (active)
        {
            update_cells();
        }
    }

private:
    void update_cells()
    {
        auto minx = std::numeric_limits<int>::max();
        auto miny = std::numeric_limits<int>::max();
        positions->iter([&](auto const& entity, auto& position) {
            const auto& velocity = *velocities->get_component(entity);
            position += velocity;
            if (position.x() < minx)
                minx = position.x();
            if (position.y() < miny)
                miny = position.y();
        });
        // translate
        const auto translate = mino::Point{{-minx, -miny}};
        positions->iter([&](auto const& entity, auto& position) { position += translate; });
        if (++count == 10000)
        {
            active = false;
            logger->info("At 10k, stopping step forward with 'E'");
        }
    }

    void init()
    {
        for (auto& [pos, vel] : DAY10_INPUT)
        {
            //
            auto entity = engine.add_entity();
            auto& velocity = velocities->add_component(entity.id);
            velocity = Velocity{{vel.x(), vel.y()}};

            auto components = render_system->create_renderable_entity(entity);

            components.position = pos;

            components.render.texture = render_system->load_texture("data/star.png");
            components.render.source = {0, 0, 256, 256};
            components.render.dest = {0, 0, 1, 1};
        }
    }
};


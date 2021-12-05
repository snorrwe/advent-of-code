#define _SILENCE_CXX17_RESULT_OF_DEPRECATION_WARNING
#include "day10input.hpp"
#include "minomaly/minomaly.hpp"
#include "minomaly/system/input_system.hpp"
#include "minomaly/system/render_system.hpp"
#include <iostream>
#include <limits>
#include <random>

const auto STAR_SIZE = 10;
const auto TRANSLATE_OFFSET = mino::Point{1920 / 2, 1080 / 2};

struct Velocity
{
    mino::Point value;
};

class MapSystem final : public mino::ISystem
{
    mino::Minomaly& engine;
    mino::RenderSystem* render_system = nullptr;
    mino::InputSystem* input_system = nullptr;
    mino::Manager<mino::PositionComponent>* positions = nullptr;
    mino::Manager<Velocity>* velocities = nullptr;
    mino::Logger* logger = nullptr;

    bool active = false;
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

        positions = engine.get_or_create_manager<mino::Manager<mino::PositionComponent>>();
        velocities = engine.get_or_create_manager<mino::Manager<Velocity>>();

        init_cells();
        update_cells(10000);
        logger->info("MapSystem started successfully\n"
                     "Start / Stop time by pressing the Spacebar. "
                     "Step forward by pressing 'E', backwards by pressing 'W'. "
                     "Exit by pressing 'Q'");
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
                    update_cells(1);
                }
                else if (event.key.keysym.sym == SDLK_w && !active)
                {
                    update_cells(-1);
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
            update_cells(1);
        }
    }

private:
    void update_cells(int times)
    {
        auto minx = std::numeric_limits<int>::max();
        auto miny = std::numeric_limits<int>::max();
        auto maxx = std::numeric_limits<int>::min();
        auto maxy = std::numeric_limits<int>::min();
        positions->iter([&](auto const& entity, auto& position) {
            const auto& velocity = *velocities->get_component(entity);
            position += velocity.value * times;
            if (position.x() > maxx)
                maxx = position.x();
            if (position.x() < minx)
                minx = position.x();
            if (position.y() > maxy)
                maxy = position.y();
            if (position.y() < miny)
                miny = position.y();
        });
        const auto translate =
            mino::Point{{-abs(maxx - minx) / 2 - minx, -abs(maxy - miny) / 2 - miny}} +
            TRANSLATE_OFFSET;
        positions->iter([&](auto const& entity, auto& position) { position += translate; });
        count += times;
        logger->info("Tick {}", count);
    }

    void init_cells()
    {
        for (auto& [pos, vel] : DAY10_INPUT)
        {
            auto entity = engine.add_entity();
            auto& velocity = velocities->add_component(entity.id);
            velocity = Velocity{vel * STAR_SIZE};

            auto components = render_system->create_renderable_entity(entity);

            components.position = pos * STAR_SIZE;

            components.render.texture = render_system->load_texture("data/star.png");
            components.render.source = {0, 0, 256, 256};
            components.render.dest = {0, 0, STAR_SIZE, STAR_SIZE};
        }
    }
};

int main(int argc, char** argv)
{
    auto engine = mino::Minomaly();
    engine.create_system<MapSystem>(engine);

    engine.run();
    return 0;
}


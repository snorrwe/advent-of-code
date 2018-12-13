#define _SILENCE_CXX17_RESULT_OF_DEPRECATION_WARNING
#include "minomaly/minomaly.hpp"
#include "minomaly/system/input_system.hpp"
#include "minomaly/system/render_system.hpp"
#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>

const auto STAR_SIZE = 20;
const auto TRANSLATE_OFFSET = mino::Point{1920 / 2, 1080 / 2};

struct Velocity
{
    mino::Point value;
};

enum class Turn : uint8_t
{
    Left = 0,
    Straight = 1,
    Right = 2,
};

enum class Type : uint8_t
{
    Horizontal,
    Vertical,
    NW,
    NE,
    Cross,
    Train,
};

class MapSystem final : public mino::ISystem
{
    mino::Minomaly& engine;
    mino::RenderSystem* render_system = nullptr;
    mino::InputSystem* input_system = nullptr;
    mino::Manager<mino::PositionComponent>* positions = nullptr;
    mino::Manager<Velocity>* velocities = nullptr;
    mino::Manager<Turn>* turns = nullptr;
    mino::Logger* logger = nullptr;
    std::vector<mino::Entity> trains = {};

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
        turns = engine.get_or_create_manager<mino::Manager<Turn>>();

        read_input();
        logger->info("MapSystem started successfully\n"
                     "Start / Stop time by pressing the Spacebar. "
                     // "Step forward by pressing 'E', backwards by pressing 'W'. "
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
                // else if (event.key.keysym.sym == SDLK_e && !active)
                // {
                //     update_trains(1);
                // }
                // else if (event.key.keysym.sym == SDLK_w && !active)
                // {
                //     update_trains(-1);
                // }
                else if (event.key.keysym.sym == SDLK_q)
                {
                    logger->info("Exiting");
                    engine.stop();
                }
            }
        }
        if (active)
        {
            update_trains(1);
        }
    }

private:
    void update_trains(int times)
    {
    }

    void read_input()
    {
        std::ifstream input("data/input.txt");
        std::string line;
        auto y = 0;
        while (std::getline(input, line))
        {
            logger->info("{}", line);
            auto x = 0;
            for (auto chr : line)
            {
                auto entity = engine.add_entity();

                auto pos = mino::Point(x, y);
                auto components = render_system->create_renderable_entity(entity);
                components.position = pos * STAR_SIZE;

                switch (chr)
                {
                case '>':
                {
                    setup_entity(entity, components, Type::Train);
                    auto& vel = velocities->add_component(entity.id);
                    vel = Velocity{mino::Point(1, 0)};
                    break;
                }
                case '<':
                {
                    setup_entity(entity, components, Type::Train);
                    auto& vel = velocities->add_component(entity.id);
                    vel = Velocity{mino::Point(-1, 0)};
                    break;
                }
                case '^':
                {
                    setup_entity(entity, components, Type::Train);
                    auto& vel = velocities->add_component(entity.id);
                    vel = Velocity{mino::Point(0, -1)};
                    break;
                }
                case 'v':
                {
                    setup_entity(entity, components, Type::Train);
                    auto& vel = velocities->add_component(entity.id);
                    vel = Velocity{mino::Point(0, 1)};
                    break;
                }
                case '-':
                    setup_entity(entity, components, Type::Horizontal);
                    break;
                case '|':
                    setup_entity(entity, components, Type::Vertical);
                    break;
                    
                case ' ':
                    // Do nothing
                    break;
                default:
                    logger->error("Unexpected character {} in input", chr);
                }
                ++x;
            }
            ++y;
        }
    }

    void setup_entity(mino::Entity const& entity, mino::RenderableEntity& components, Type type)
    {
        switch (type)
        {
        case Type::Train:
            components.render.texture = render_system->load_texture("data/star.png");
            components.render.source = {0, 0, 256, 256};
            components.render.dest = {0, 0, STAR_SIZE, STAR_SIZE};
            break;
        case Type::Horizontal:
            components.render.texture = render_system->load_texture("data/star.png");
            components.render.source = {0, 125, 256, 127};
            components.render.dest = {0, 0, STAR_SIZE, STAR_SIZE / 10};
            break;
        case Type::Vertical:
            components.render.texture = render_system->load_texture("data/star.png");
            components.render.source = {125, 0, 127, 256};
            components.render.dest = {0, 0, STAR_SIZE / 10, STAR_SIZE};
            break;
        default:
            break;
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


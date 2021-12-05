#define _SILENCE_CXX17_RESULT_OF_DEPRECATION_WARNING
#include "minomaly/minomaly.hpp"
#include "minomaly/system/input_system.hpp"
#include "minomaly/system/render_system.hpp"
#include <iostream>
#include <limits>
#include <random>

const auto CELL_SIZE = 10;
const auto TRANSLATE_OFFSET = mino::Point{1920 / 2, 1080 / 2};


class MapSystem final : public mino::ISystem
{
    mino::Minomaly& engine;
    mino::RenderSystem* render_system = nullptr;
    mino::InputSystem* input_system = nullptr;
    mino::Manager<mino::PositionComponent>* positions = nullptr;
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

        logger->info("MapSystem started successfully\n"
                "Start / Stop time by pressing the Spacebar. "
                "Step forward by pressing 'E', backwards by pressing 'W'. "
                "Exit by pressing 'Q'");
    }

    void update() override
    {
    }

    private:
};

int main(int argc, char** argv)
{
    auto engine = mino::Minomaly();
    engine.create_system<MapSystem>(engine);

    engine.run();
    return 0;
}


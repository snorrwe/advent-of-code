#define _SILENCE_CXX17_RESULT_OF_DEPRECATION_WARNING
#include "minomaly/minomaly.hpp"
#include "map_system.hpp"

int main(int argc, char** argv)
{
    auto engine = mino::Minomaly();
    engine.create_system<MapSystem>(engine);

    engine.run();
    return 0;
}


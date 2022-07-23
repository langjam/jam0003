#pragma once

#include <cstddef>

#include "value.h"

template <std::size_t width, std::size_t height>
class Grid {
   private:
    Command grid[height][width];

   public:
    Grid();

    void set(std::size_t x, std::size_t y, Command command);

    Command get(std::size_t x, std::size_t y);
};

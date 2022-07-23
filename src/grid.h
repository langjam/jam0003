#pragma once

#include <cstddef>

#include "value.h"

class Grid {
   private:
    Command* grid_;
    size_t height_;
    size_t width_;

   public:
    Grid(size_t width, size_t height);
    ~Grid();

    void set(size_t x, size_t y, Command command);

    Command get(size_t x, size_t y);
};

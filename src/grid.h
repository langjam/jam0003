#pragma once

#include <cstddef>

#include "value.h"

class Grid {
   public:
    Grid(size_t width, size_t height);
    ~Grid();

    void set(size_t x, size_t y, Command command);

    Command get(size_t x, size_t y);

    size_t width();
    size_t height();

   private:
    size_t width_;
    size_t height_;
    Command* grid_;
};

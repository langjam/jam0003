#include "grid.h"

#include <runtime/values/value.h>

#include <memory>

Grid::Grid(size_t width, size_t height) : width_(width), height_(height) {
    grid_ = new Command[height * width];
}

Grid::~Grid() { delete[] grid_; }

void Grid::set(size_t x, size_t y, Command command) {
    grid_[y * width_ + x] = command;
}

Command Grid::get(size_t x, size_t y) { return grid_[y * width_ + x]; }

size_t Grid::width() { return width_; }

size_t Grid::height() { return height_; }

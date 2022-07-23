#include "grid.h"

#include "value.h"

void Grid::set(size_t x, size_t y, Command command) {}

Command Grid::get(size_t x, size_t y) { return grid[y][x]; }

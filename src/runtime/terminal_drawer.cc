#include "terminal_drawer.h"

#include <runtime/drawer.h>
#include <runtime/grid.h>
#include <runtime/value.h>

void TerminalDrawer::draw(Grid* grid) {
    for (size_t y = grid->height(); y > 0; y--) {
        for (size_t x = 0; x < grid->width(); x++) {
            grid->get(y, x).display();
        }
    }
}

#pragma once

#include <runtime/drawer.h>

class TerminalDrawer : public Drawer {
   private:
   public:
    TerminalDrawer();

    void draw(Grid* grid);
};

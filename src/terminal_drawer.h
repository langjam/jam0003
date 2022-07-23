#pragma once

#include "drawer.h"

class TerminalDrawer : public Drawer {
   private:
   public:
    TerminalDrawer();

    void draw(Grid* grid);
};

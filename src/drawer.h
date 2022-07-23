#pragma once

#include "grid.h"

class Drawer {
   private:
   public:
    Drawer();

    virtual void draw(Grid* grid) = 0;
};

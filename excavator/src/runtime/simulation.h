#pragma once

#include <curses.h>
#include <runtime/command.h>

class Simulation {
   public:
    enum Stage { Load = 1, DrawGraph, EmptyScreen, DrawPainting, End };
    Simulation(const std::vector<CommandCell>& commands);
    ~Simulation();

    void set_running(bool running) { m_running = running; }
    void paint() {
        mvaddch(m_y, m_x * 2, '0');
    }
    void move(int rel_x, int rel_y) {
        m_x += rel_x;
        rel_y += rel_y;
    }

    void run();

   private:
    const std::vector<CommandCell>& m_commands;
    WINDOW* m_window;
    int m_x, m_y;
    bool m_running{true};

    bool running() { return m_running; }
};

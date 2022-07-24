#include "simulation.h"

#include <iostream>

Simulation::Simulation(const std::vector<CommandCell>& commands)
    : m_commands(commands) {
    // Setup curses
    m_window = initscr();
    noecho();
    beep();
    getmaxyx(m_window, m_y, m_x);
    m_y /= 2;
    m_x /= 2;
    keypad(m_window, TRUE);
    clear();
}

Simulation::~Simulation() {
    endwin();
}

void Simulation::run() {
    curs_set(false);
    size_t graph_command_index = 0;
    while (running()) {
        auto event = getch();
        switch (event) {
        case 'q':
            set_running(false);
            break;
        case ' ':
            if (graph_command_index < m_commands.size()) {
                // Show command
                CommandCell command = m_commands[graph_command_index];
                switch (command.type()) {
                case GoLeft:
                    m_x -= 2;
                    break;
                case GoRight:
                    m_x += 2;
                    break;
                case GoUp:
                    m_y -= 1;
                    break;
                case GoDown:
                    m_y += 1;
                    break;
                default:
                    break;
                }
                mvaddch(m_y, m_x, '#');
                ++graph_command_index;
            } else {
                set_running(false);
            }
            break;
        }
    }
}

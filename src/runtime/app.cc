#include "app.h"

#include <QMainWindow>
#include <QTableView>
#include <QAbstractTableModel>

TableModel::TableModel() : QAbstractTableModel() {
}

JamWindow::JamWindow() : QMainWindow() {
    setWindowTitle("Jam Window");
    m_table_model = new TableModel;
    m_table = new QTableView;
    m_table->setShowGrid(false);
    m_table->setModel(m_table_model);

    setCentralWidget(m_table);
    show();
}

JamApp::JamApp(int argc, char* argv[]) : QApplication(argc, argv) {
    m_window = new JamWindow;    
}

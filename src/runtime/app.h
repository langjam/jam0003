#pragma once

#include <QApplication>
#include <QMainWindow>
#include <QTableView>
#include <QVariant>

class TableModel : public QAbstractTableModel {
public:
    TableModel();

    int rowCount(const QModelIndex &parent = QModelIndex()) const { return 100; }
    int columnCount(const QModelIndex &parent = QModelIndex()) const { return 100; }
    QVariant data(const QModelIndex &index, int role = Qt::DisplayRole) const { return QVariant::Invalid; }
};

class JamWindow : public QMainWindow {
public:
    JamWindow();

private:
    TableModel* m_table_model { nullptr };
    QTableView* m_table { nullptr };
};

class JamApp : public QApplication {
public:
    JamApp(int argc, char* argv[]);

private:
    JamWindow* m_window;
};

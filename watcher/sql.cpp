#include <iostream>
#include <string>
#include "sqlite3.h"


struct Sqlite{
    sqlite3* db = nullptr;
    char* errMsg = nullptr;
    int rc;
    Sqlite(std::string db_path){
        rc = sqlite3_open(db_path.c_str(), &db);
        if (rc != SQLITE_OK) {
            std::cerr << "Can't open database: " 
                  << sqlite3_errmsg(db) << std::endl;
        }
    }
    ~Sqlite(){
        sqlite3_close(db);
    }

    void exec(std::string sql){
        rc = sqlite3_exec(db, sql.c_str(), callback, nullptr, &errMsg);
        if (rc != SQLITE_OK) {
            std::cerr << "SQL error: " << (errMsg ? errMsg : "Unknown error") << std::endl;
            sqlite3_free(errMsg);  // Free the error message allocated by SQLite.
            errMsg = nullptr;
        }
    }

    static int callback(void* /*unused*/, int argc, char** argv, char** azColName) {
    for (int i = 0; i < argc; i++) {
        // Print column name and value; if value is NULL, display "NULL"
        std::cout << (azColName[i] ? azColName[i] : "NULL")
                  << " = " << (argv[i] ? argv[i] : "NULL") << "\n";
    }
    std::cout << std::endl;  // Extra newline between rows.
    return 0;
    }

};
// Callback function for sqlite3_exec.
// This function is called for each row returned by a query.

int main() {

}

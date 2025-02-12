#include <windows.h>
#include <iostream>
#include <vector>
#include <string>
#include <thread>

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
        //std::cout << (azColName[i] ? azColName[i] : "NULL") << " = " << (argv[i] ? argv[i] : "NULL") << "\n";
    }
    //std::cout << std::endl;  // Extra newline between rows.
    return 0;
    }

}; 

SERVICE_STATUS ServiceStatus;
SERVICE_STATUS_HANDLE hStatus;

std::string directoryToWatch = "C:\\path\\to\\watch";  // Default path
std::string db_path = "C:\\Users\\jimts\\ytp\\sortifile\\watcher\\example.db";

// Function to monitor the directory
void WatchDirectory(const std::string& directory) {
    Sqlite db(db_path);
    HANDLE hDir = CreateFileA(
        directory.c_str(),
        FILE_LIST_DIRECTORY,
        FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE,
        nullptr,
        OPEN_EXISTING,
        FILE_FLAG_BACKUP_SEMANTICS,
        nullptr
    );

    if (hDir == INVALID_HANDLE_VALUE) {
        return;
    }

    const DWORD bufferSize = 1024 * 10;
    std::vector<BYTE> buffer(bufferSize);
    DWORD bytesReturned;

    while (true) {
        BOOL success = ReadDirectoryChangesW(
            hDir,
            buffer.data(),
            bufferSize,
            TRUE,
            FILE_NOTIFY_CHANGE_FILE_NAME |
            FILE_NOTIFY_CHANGE_DIR_NAME |
            FILE_NOTIFY_CHANGE_ATTRIBUTES |
            FILE_NOTIFY_CHANGE_SIZE |
            FILE_NOTIFY_CHANGE_LAST_WRITE,
            &bytesReturned,
            nullptr,
            nullptr
        );

        if (!success) {
            break;
        }

        DWORD offset = 0;
        do {
            FILE_NOTIFY_INFORMATION* pNotify =
                reinterpret_cast<FILE_NOTIFY_INFORMATION*>(buffer.data() + offset);

            int length = WideCharToMultiByte(CP_ACP, 0, pNotify->FileName,
                                             pNotify->FileNameLength / sizeof(WCHAR),
                                             nullptr, 0, nullptr, nullptr);
            std::string fileName(length, '\0');
            WideCharToMultiByte(CP_ACP, 0, pNotify->FileName,
                                pNotify->FileNameLength / sizeof(WCHAR),
                                &fileName[0], length, nullptr, nullptr);

            switch (pNotify->Action) {
                case FILE_ACTION_ADDED:
                    //std::cout << "File added: " << fileName << std::endl;
                    db.exec("INSERT INTO lis (data) VALUES ('File added: " + fileName + "');");
                    break;
                case FILE_ACTION_REMOVED:
                    //std::cout << "File removed: " << fileName << std::endl;
                    db.exec("INSERT INTO lis (data) VALUES ('File removed: " + fileName + "');");
                    break;
                case FILE_ACTION_MODIFIED:
                    //std::cout << "File modified: " << fileName << std::endl;
                    db.exec("INSERT INTO lis (data) VALUES ('File modified: " + fileName + "');");
                    break;
                case FILE_ACTION_RENAMED_OLD_NAME:
                    //std::cout << "File renamed (old name): " << fileName << std::endl;
                    db.exec("INSERT INTO lis (data) VALUES ('File renamed (old name): " + fileName + "');");
                    break;
                case FILE_ACTION_RENAMED_NEW_NAME:
                    //std::cout << "File renamed (new name): " << fileName << std::endl;
                    db.exec("INSERT INTO lis (data) VALUES ('File renamed (new name): " + fileName + "');");
                    break;
            }

            if (pNotify->NextEntryOffset == 0)
                break;
            offset += pNotify->NextEntryOffset;
        } while (offset < bytesReturned);
    }

    CloseHandle(hDir);
}

// Service Control Handler
void WINAPI ServiceCtrlHandler(DWORD CtrlCode) {
    switch (CtrlCode) {
        case SERVICE_CONTROL_STOP:
            ServiceStatus.dwCurrentState = SERVICE_STOPPED;
            SetServiceStatus(hStatus, &ServiceStatus);
            return;
        default:
            break;
    }

    SetServiceStatus(hStatus, &ServiceStatus);
}

// Service Main Function
void WINAPI ServiceMain(DWORD argc, LPTSTR* argv) {
    ServiceStatus.dwServiceType = SERVICE_WIN32;
    ServiceStatus.dwCurrentState = SERVICE_START_PENDING;
    ServiceStatus.dwControlsAccepted = SERVICE_ACCEPT_STOP;
    ServiceStatus.dwWin32ExitCode = 0;
    ServiceStatus.dwServiceSpecificExitCode = 0;
    ServiceStatus.dwCheckPoint = 0;
    ServiceStatus.dwWaitHint = 0;

    hStatus = RegisterServiceCtrlHandler("FileWatcherService", ServiceCtrlHandler);
    if (!hStatus) return;

    ServiceStatus.dwCurrentState = SERVICE_RUNNING;
    SetServiceStatus(hStatus, &ServiceStatus);

    WatchDirectory(directoryToWatch);

    ServiceStatus.dwCurrentState = SERVICE_STOPPED;
    SetServiceStatus(hStatus, &ServiceStatus);
}

int main(int argc, char* argv[]) {
    if (argc == 3) {
        directoryToWatch = argv[1];
        db_path = argv[2];
    }
    auto db = Sqlite(db_path);
    db.exec("CREATE TABLE IF NOT EXISTS lis (id INTEGER PRIMARY KEY, data TEXT);");

    // WatchDirectory(directoryToWatch);

    
    SERVICE_TABLE_ENTRY ServiceTable[] = {
        {"FileWatcherService", (LPSERVICE_MAIN_FUNCTION)ServiceMain},
        {nullptr, nullptr}
    };

    StartServiceCtrlDispatcher(ServiceTable);
    //db = Sqlite("example.db");
    

    return 0;
}
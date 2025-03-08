#include <windows.h>
#include <iostream>
#include <vector>
#include <string>
#include <thread>
#include <cstring>
#include <cstdio>  // For sprintf

#include "sqlite3.h"

// Helper function to get the current timestamp as a string.
std::string GetCurrentTimestamp() {
    SYSTEMTIME st;
    GetLocalTime(&st);
    char buffer[100];
    sprintf(buffer, "%04d-%02d-%02d %02d:%02d:%02d",
            st.wYear, st.wMonth, st.wDay, st.wHour, st.wMinute, st.wSecond);
    return std::string(buffer);
}

struct Sqlite {
    sqlite3* db = nullptr;
    char* errMsg = nullptr;
    int rc;
    Sqlite(std::string db_path) {
        rc = sqlite3_open(db_path.c_str(), &db);
        if (rc != SQLITE_OK) {
            std::cerr << "Can't open database: " 
                      << sqlite3_errmsg(db) << std::endl;
        }
    }
    ~Sqlite() {
        sqlite3_close(db);
    }

    void exec(std::string sql) {
        rc = sqlite3_exec(db, sql.c_str(), callback, nullptr, &errMsg);
        if (rc != SQLITE_OK) {
            std::cerr << "SQL error: " << (errMsg ? errMsg : "Unknown error") << std::endl;
            sqlite3_free(errMsg);  // Free the error message allocated by SQLite.
            errMsg = nullptr;
        }
    }

    static int callback(void* /*unused*/, int argc, char** argv, char** azColName) {
        for (int i = 0; i < argc; i++) {
            // Uncomment if you want to print query results.
            // std::cout << (azColName[i] ? azColName[i] : "NULL") << " = " 
            //           << (argv[i] ? argv[i] : "NULL") << "\n";
        }
        // std::cout << std::endl;
        return 0;
    }
};

SERVICE_STATUS ServiceStatus;
SERVICE_STATUS_HANDLE hStatus;

std::string directoryToWatch = "C:\\path\\to\\watch";  // Default path
std::string db_path = "C:\\Users\\jimts\\ytp\\sortifile\\watcher\\example.db";

// Helper function to get the file ID given its full path.
ULONGLONG GetFileId(const std::string& fullPath) {
    // Open the file with minimal access
    HANDLE hFile = CreateFileA(fullPath.c_str(),
                               0,  // no access required, just for metadata
                               FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE,
                               nullptr,
                               OPEN_EXISTING,
                               FILE_FLAG_BACKUP_SEMANTICS,
                               nullptr);
    if (hFile == INVALID_HANDLE_VALUE) {
        return 0; // File may not exist (e.g., if it was removed) or an error occurred.
    }

    BY_HANDLE_FILE_INFORMATION fileInfo;
    ULONGLONG fileId = 0;
    if (GetFileInformationByHandle(hFile, &fileInfo)) {
        fileId = (static_cast<ULONGLONG>(fileInfo.nFileIndexHigh) << 32) |
                  fileInfo.nFileIndexLow;
    }
    CloseHandle(hFile);
    return fileId;
}

void WatchDirectory(const std::string& directory) {
    // Create or open the database and ensure the table has all desired columns.
    Sqlite db(db_path);
    db.exec("CREATE TABLE IF NOT EXISTS lis ("
            "id INTEGER PRIMARY KEY, "
            "src_path TEXT, "
            "new_path TEXT, "
            "move_timestamp TEXT, "
            "moved_by TEXT, "
            "reason TEXT DEFAULT 'none', "
            "fileID INTEGER, "
            "event_type TEXT"
            ");");

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
        std::cerr << "Failed to open directory: " << directory << std::endl;
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
            std::cerr << "Error reading directory changes." << std::endl;
            break;
        }

        DWORD offset = 0;
        do {
            FILE_NOTIFY_INFORMATION* pNotify =
                reinterpret_cast<FILE_NOTIFY_INFORMATION*>(buffer.data() + offset);

            // Convert the wide-character file name to a multi-byte string.
            int length = WideCharToMultiByte(CP_ACP, 0, pNotify->FileName,
                                             pNotify->FileNameLength / sizeof(WCHAR),
                                             nullptr, 0, nullptr, nullptr);
            std::string fileName(length, '\0');
            WideCharToMultiByte(CP_ACP, 0, pNotify->FileName,
                                pNotify->FileNameLength / sizeof(WCHAR),
                                &fileName[0], length, nullptr, nullptr);

            // Construct the full path.
            std::string fullPath = directory;
            if (!fullPath.empty() && fullPath.back() != '\\') {
                fullPath += "\\";
            }
            fullPath += fileName;

            // Retrieve the file ID.
            ULONGLONG fileId = GetFileId(fullPath);

            // Get the current timestamp.
            std::string timestamp = GetCurrentTimestamp();

            // Default values for moved_by and reason.
            std::string moved_by = "unknown";
            std::string reason = "none";

            // Determine event type and set src_path/new_path accordingly.
            std::string src_path;
            std::string new_path;
            std::string event_type;

            switch (pNotify->Action) {
                case FILE_ACTION_ADDED:
                    src_path = fullPath;
                    new_path = "";
                    event_type = "added";
                    break;
                case FILE_ACTION_REMOVED:
                    src_path = fullPath;
                    new_path = "";
                    event_type = "removed";
                    break;
                case FILE_ACTION_MODIFIED:
                    src_path = fullPath;
                    new_path = "";
                    event_type = "modify";
                    break;
                case FILE_ACTION_RENAMED_OLD_NAME:
                    // For a rename, we record the old name.
                    src_path = fullPath;
                    new_path = "";
                    event_type = "move";
                    break;
                case FILE_ACTION_RENAMED_NEW_NAME:
                    // For a rename, we record the new name.
                    src_path = "";
                    new_path = fullPath;
                    event_type = "move";
                    break;
                default:
                    src_path = fullPath;
                    new_path = "";
                    event_type = "unknown";
                    break;
            }

            // Prepare the SQL insert statement using a prepared statement.
            const char* insert_sql = "INSERT INTO lis (src_path, new_path, move_timestamp, moved_by, reason, fileID, event_type) VALUES (?, ?, ?, ?, ?, ?, ?);";
            sqlite3_stmt* stmt;
            if (sqlite3_prepare_v2(db.db, insert_sql, -1, &stmt, nullptr) == SQLITE_OK) {
                sqlite3_bind_text(stmt, 1, src_path.c_str(), -1, SQLITE_TRANSIENT);
                sqlite3_bind_text(stmt, 2, new_path.c_str(), -1, SQLITE_TRANSIENT);
                sqlite3_bind_text(stmt, 3, timestamp.c_str(), -1, SQLITE_TRANSIENT);
                sqlite3_bind_text(stmt, 4, moved_by.c_str(), -1, SQLITE_TRANSIENT);
                sqlite3_bind_text(stmt, 5, reason.c_str(), -1, SQLITE_TRANSIENT);
                sqlite3_bind_int64(stmt, 6, static_cast<sqlite3_int64>(fileId));
                sqlite3_bind_text(stmt, 7, event_type.c_str(), -1, SQLITE_TRANSIENT);

                if (sqlite3_step(stmt) != SQLITE_DONE) {
                    std::cerr << "Error inserting data: " << sqlite3_errmsg(db.db) << std::endl;
                }
            } else {
                std::cerr << "Failed to prepare statement: " << sqlite3_errmsg(db.db) << std::endl;
            }
            sqlite3_finalize(stmt);

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
// Service Main Function
void WINAPI ServiceMain(DWORD argc, LPTSTR* argv) {
    ServiceStatus.dwServiceType = SERVICE_WIN32_OWN_PROCESS;
    ServiceStatus.dwCurrentState = SERVICE_START_PENDING;
    ServiceStatus.dwControlsAccepted = SERVICE_ACCEPT_STOP;
    ServiceStatus.dwWin32ExitCode = 0;
    ServiceStatus.dwServiceSpecificExitCode = 0;
    ServiceStatus.dwCheckPoint = 0;
    ServiceStatus.dwWaitHint = 0;

    hStatus = RegisterServiceCtrlHandler("FileWatcherService", ServiceCtrlHandler);
    if (!hStatus) {
        return;
    }

    // Initialize the service and set it to running.
    ServiceStatus.dwCurrentState = SERVICE_RUNNING;
    SetServiceStatus(hStatus, &ServiceStatus);

    // Start the directory watching in a separate thread.
    std::thread watcherThread(WatchDirectory, directoryToWatch);
    watcherThread.detach();  // Detach the thread to allow it to run independently.

    // Keep the main thread running to prevent the service from exiting.
    // You can use a condition variable or another synchronization mechanism here.
    while (ServiceStatus.dwCurrentState == SERVICE_RUNNING) {
        Sleep(1000);  // Sleep to reduce CPU usage.
    }
}


int main(int argc, char* argv[]) {
    if (argc == 3) {
        directoryToWatch = argv[1];
        db_path = argv[2];
    }
    Sqlite db(db_path);
    db.exec("CREATE TABLE IF NOT EXISTS move_history ("
            "id INTEGER PRIMARY KEY, "
            "src_path TEXT, "
            "new_path TEXT, "
            "move_timestamp TEXT, "
            "moved_by TEXT, "
            "reason TEXT DEFAULT 'none', "
            "fileID INTEGER, "
            "event_type TEXT"
            ");");

    // Uncomment the following line to run the directory watcher in standalone mode.
    // WatchDirectory(directoryToWatch);

    SERVICE_TABLE_ENTRY ServiceTable[] = {
        {"FileWatcherService", (LPSERVICE_MAIN_FUNCTION)ServiceMain},
        {nullptr, nullptr}
    };

    StartServiceCtrlDispatcher(ServiceTable);

    return 0;
}

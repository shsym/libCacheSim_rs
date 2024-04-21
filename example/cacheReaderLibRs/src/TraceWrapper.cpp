#include "TraceWrapper.h"
// lock api
#include <mutex>

// Initialize the static member
#define MAX_NUM_READER 64
static reader_t* reader[MAX_NUM_READER] = { nullptr };
// define lock
std::mutex reader_mtx;

int get_next_reader() {
    int idx = 0;
    for (idx = 0; idx < MAX_NUM_READER; idx ++) {
        if (reader[idx] == nullptr) {
            return idx;
        }
    }
    return -1;
}

extern "C" {
    int open_trace_oracle(const char* path) {
        // Lock the reader array
        std::lock_guard<std::mutex> lock(reader_mtx);
        int reader_idx = get_next_reader();
        if (reader_idx == -1) {
            // unlock
            return -1;
        }

        // reader_init_param_t reader_init_params;
        // memset(&reader_init_params, 0, sizeof(reader_init_params));
        // reader_init_params.ignore_size_zero_req = true;
        // reader_init_params.obj_id_is_num = true;
        // reader_init_params.sampler = NULL;

        // Attempt to open the trace using the provided library call
        reader_t *new_reader = setup_reader(path, ORACLE_GENERAL_TRACE, NULL);
        if (new_reader != nullptr) {
            printf("Reader Idx: %d\n", reader_idx);
            reader[reader_idx] = new_reader;
            printf("ZSTD reader: %p\n", reader[reader_idx]->zstd_reader_p);
            return reader_idx;
        }
        return -1;
    }

    request_t get_next_request(int reader_idx) {
        request_t req;
        req.valid = false;

        if (reader[reader_idx] == nullptr) {
            return req;
        }

        // Allocate a new request object
        if (read_one_req(reader[reader_idx], &req) != 0) {
            // Assume read_one_req returns 0 on success
            // delete req;
            return req;
        }
        // print_request(&req);
        return req;
    }

    bool close_trace_wrapper(int reader_idx) {
        // check if reader_idx is valid
        if (reader_idx < 0 || reader_idx >= MAX_NUM_READER) {
            return false;
        }
        std::lock_guard<std::mutex> lock(reader_mtx);
        // if reader is not initialized
        if (reader[reader_idx] == nullptr) {
            return false;
        }

        int result = close_reader(reader[reader_idx]);
        reader[reader_idx] = nullptr;
        return result == 0;  // Assume close_reader returns 0 on success
    }
}

#ifndef TRACE_WRAPPER_H
#define TRACE_WRAPPER_H

#include "libCacheSim.h"

class TraceWrapper {
public:
    static int open_trace_oracle(const char* path);
    static request_t get_next_request(int reader_idx);
    static bool close_trace_wrapper(int reader_idx);
};

#endif // TRACE_WRAPPER_H

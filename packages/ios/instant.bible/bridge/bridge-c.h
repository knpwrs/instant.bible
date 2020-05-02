#ifndef bridge_c_header
#define bridge_c_header

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct IbRustBuffer {
    int64_t len;
    uint8_t *_Nullable data;
} IbRustBuffer;

void bridge_init(const uint8_t *raw_data, uintptr_t len);

IbRustBuffer bridge_search(const char *bytes);

void bridge_search_free(IbRustBuffer buf);

#endif

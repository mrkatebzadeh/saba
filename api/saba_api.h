/**
 * api/saba_api.h
 * Copyright (c) 2021 M.R. Siavash Katebzadeh <mr.katebzadeh@gmail.com>
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

#ifndef SABA_API_H
#define SABA_API_H

#ifdef __cplusplus
#include <cstdint>
extern "C" {
#endif

enum SABA_RESULT { SUCCESSFUL, FAILED };

// Application
SABA_RESULT saba_app_register(const char *application_name,
                              uint32_t *application_fd);
SABA_RESULT saba_app_deregister(const uint32_t *application_fd);

// Connection
SABA_RESULT saba_connection_create(uint32_t *connection_fd,
                                   const char *destination_ip, int32_t port,
                                   const uint32_t *application_fd);
SABA_RESULT saba_connection_destroy(int connection_fd);
SABA_RESULT saba_connection_establish(int connection_fd);

// Memory
SABA_RESULT saba_memory_allocate(int connection_fd, uint8_t *memory,
                                 uint32_t len);
SABA_RESULT saba_memory_free(int connection_fd, uint8_t *memory,
                                   uint32_t len);

// Exchange data
SABA_RESULT saba_write(int connection_fd, uint8_t *memory, uint32_t len);
SABA_RESULT saba_read(int connection_fd, uint8_t *memory, uint32_t len);

// Error message
void saba_result_getstring(SABA_RESULT result, char* result_str);
void saba_result_print(SABA_RESULT result, char* result_str);

#ifdef __cplusplus
}
#endif

#endif

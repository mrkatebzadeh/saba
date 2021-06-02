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

class SabaResult {
public:
  enum { SUCCESSFUL, FAILED };
  static const char *getstring(int err) {
    switch (err) {
    case SabaResult::SUCCESSFUL:
      return "Successful";
    case SabaResult::FAILED:
      return "Failed";
    default:
      return "No such error number";
    }
  }
};

// Application
int saba_app_register(const char *application_name, uint32_t *application_fd);
int saba_app_deregister(uint32_t *application_fd);

// Connection
int saba_connection_create(uint32_t *connection_fd, const char *destination_ip,
                           int16_t port, const uint32_t *application_fd);
int saba_connection_destroy(uint32_t connection_fd);
int saba_connection_establish(uint32_t connection_fd);
int saba_connection_create_server(uint32_t connection_fd, const char *local_ip,
                           int16_t port, const uint32_t *application_fd);

// Memory
int saba_memory_allocate(uint32_t connection_fd, uint8_t **memory, uint32_t len);
int saba_memory_free(uint32_t connection_fd, uint8_t **memory, uint32_t len);

// Exchange data
int saba_write(uint32_t connection_fd, uint8_t *memory, uint32_t len);
int saba_read(uint32_t connection_fd, uint8_t *memory, uint32_t len);

// Error message
void saba_result_getstring(int result, char *result_str);
void saba_result_print(int result);

#ifdef __cplusplus
}
#endif

#endif

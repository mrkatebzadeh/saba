/**
 * api/saba_socket.cpp
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

#include "saba_api.h"
#include <arpa/inet.h>
#include <cstdint>
#include <cstdio>
#include <cstdlib>
#include <sys/socket.h>
#include <vector>

class SocketResult : public SabaResult {
public:
  enum {
    SOCKET_INVALID_FD,
    SOCKET_CREATE_FAILED,
    SOCKET_CONNECT_FAILED,
    SOCKET_SEND_FAILED,
    SOCKET_RECV_FAILED,
    SOCKET_ALLOC_FAILED
  };
  static const char *getstring(int err) {
    switch (err) {
    case SOCKET_INVALID_FD:
      return "Socket invalid fd";
    case SOCKET_CREATE_FAILED:
      return "Socket create failed";
    case SOCKET_CONNECT_FAILED:
      return "Socket connect failed";
    case SOCKET_SEND_FAILED:
      return "Socket send failed";
    case SOCKET_RECV_FAILED:
      return "Socket recv failed";
    case SOCKET_ALLOC_FAILED:
      return "Socket alloc failed";
    default:
      return SabaResult::getstring(err);
    }
  }
};

class SocketConnection {
public:
  int socket_desc;
  struct sockaddr_in destination;
  std::uint32_t memory_size;
  bool valid;

  int create_socket(const char *address, std::uint16_t port) {
    this->socket_desc = socket(AF_INET, SOCK_STREAM, 0);

    if (this->socket_desc == -1) {
      return SocketResult::SOCKET_CREATE_FAILED;
    }

    this->destination.sin_addr.s_addr = inet_addr(address);
    this->destination.sin_family = AF_INET;
    this->destination.sin_port = htons(port);

    this->valid = false;
    return SocketResult::SUCCESSFUL;
  }

  int connect_socket() {
    if (connect(this->socket_desc, (struct sockaddr *)&this->destination,
                sizeof(this->destination)) < 0) {
      return SocketResult::SOCKET_CONNECT_FAILED;
    }
    this->valid = true;
    return SocketResult::SUCCESSFUL;
  }

  int send_data(char *message, uint32_t len) {
    if (send(this->socket_desc, message, len, 0) < 0) {
      return SocketResult::SOCKET_SEND_FAILED;
    }
    return SocketResult::SUCCESSFUL;
  }

  int recieve_data(char *message, uint32_t len) {
    if (recv(this->socket_desc, message, len, 0) < 0) {
      return SocketResult::SOCKET_RECV_FAILED;
    }
    return SocketResult::SUCCESSFUL;
  }
};

static std::vector<SocketConnection *> sockets;
// Application
int saba_app_register(const char *application_name, uint32_t *application_fd) {
  *application_fd = 1; // TODO
  return SocketResult::SUCCESSFUL;
}

int saba_app_deregister(uint32_t *application_fd) {

  *application_fd = 0; // TODO
  return SocketResult::SUCCESSFUL;
}

// Connection
int saba_connection_create(uint32_t *connection_fd, const char *destination_ip,
                           int16_t port, const uint32_t *application_fd) {
  *connection_fd = sockets.size();
  auto new_socket = new SocketConnection();
  if (int err = new_socket->create_socket(destination_ip, port) !=
                SocketResult::SUCCESSFUL) {
    return err;
  }
  sockets.push_back(new_socket);
  return SocketResult::SUCCESSFUL;
}

int saba_connection_destroy(int connection_fd) {
  if (sockets.size() <= connection_fd) {
    return SocketResult::SOCKET_INVALID_FD;
  }
  sockets[connection_fd]->valid = false;
  return SocketResult::SUCCESSFUL;
}

int saba_connection_establish(int connection_fd) {
  if (sockets.size() <= connection_fd) {
    return SocketResult::SOCKET_INVALID_FD;
  }
  return sockets[connection_fd]->connect_socket();
}

// Memory
int saba_memory_allocate(int connection_fd, uint8_t **memory, uint32_t len) {

  if (sockets.size() <= connection_fd) {
    return SocketResult::SOCKET_INVALID_FD;
  }
  *memory = (uint8_t *)std::malloc(len);
  if (!*memory) {
    return SocketResult::SOCKET_ALLOC_FAILED;
  }
  return SocketResult::SUCCESSFUL;
}

int saba_memory_free(int connection_fd, uint8_t **memory, uint32_t len) {

  if (sockets.size() <= connection_fd) {
    return SocketResult::SOCKET_INVALID_FD;
  }
  std::free(*memory);
  return SocketResult::SUCCESSFUL;
}

// Exchange data
int saba_write(int connection_fd, uint8_t *memory, uint32_t len) {

  if (sockets.size() <= connection_fd) {
    return SocketResult::SOCKET_INVALID_FD;
  }
  return sockets[connection_fd]->send_data((char *)memory, len);
}

int saba_read(int connection_fd, uint8_t *memory, uint32_t len) {
  if (sockets.size() <= connection_fd) {
    return SocketResult::SOCKET_INVALID_FD;
  }
  return sockets[connection_fd]->recieve_data((char *)memory, len);
}

// Error message
void saba_result_getstring(int result, char *result_str) {}

void saba_result_print(int result, char *result_str) {
  std::printf("Error: %s\n", SocketResult::getstring(result));
}

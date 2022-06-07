/**
 * test/socket_test.cpp
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

#include <gtest/gtest.h>

#include "saba.h"

class SocketTest : public ::testing::Test {
 protected:
  std::uint32_t app_fd;
  std::uint32_t conn_fd;
};

TEST_F(SocketTest, AppRegister) {
  auto result = saba_app_register("SocketTest", &app_fd);
  EXPECT_EQ(app_fd, 1);
  ASSERT_EQ(result, SabaResult::SUCCESSFUL);
}

TEST_F(SocketTest, ConnectionCreate) {
  auto local_ip = "0.0.0.0";
  int16_t port = 8989;
  auto result = saba_connection_create(&conn_fd, local_ip, port, &app_fd);
  EXPECT_EQ(conn_fd, 0);
  ASSERT_EQ(result, SabaResult::SUCCESSFUL);
}

TEST_F(SocketTest, ConnectionDestroy) {
  auto result = saba_connection_destroy(conn_fd);
  ASSERT_EQ(result, SabaResult::SUCCESSFUL);
}

TEST_F(SocketTest, AppDeregister) {
  auto result = saba_app_deregister(&app_fd);
  EXPECT_EQ(app_fd, 0);
  ASSERT_EQ(result, SabaResult::SUCCESSFUL);
}

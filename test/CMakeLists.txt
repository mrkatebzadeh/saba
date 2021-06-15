# CMakeLists
# Copyright (c) 2021 M.R. Siavash Katebzadeh <mr.katebzadeh@gmail.com>
#
# Permission is hereby granted, free of charge, to any person obtaining a copy
# of this software and associated documentation files (the "Software"), to deal
# in the Software without restriction, including without limitation the rights
# to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
# copies of the Software, and to permit persons to whom the Software is
# furnished to do so, subject to the following conditions:
#
# The above copyright notice and this permission notice shall be included in
# all copies or substantial portions of the Software.
#
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
# IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
# FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
# AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
# LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
# OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
# SOFTWARE.

set(HEADER_LIST "${Saba_SOURCE_DIR}/include/saba.h")

include(FetchContent)
FetchContent_Declare(
	googletest
	URL https://github.com/google/googletest/archive/609281088cfefc76f9d0ce82e1ff6c30cc3591e5.zip
)
FetchContent_MakeAvailable(googletest)

FetchContent_Declare(
        rpc
        GIT_REPOSITORY https://github.com/rpclib/rpclib.git
        GIT_TAG        v2.2.1
)

FetchContent_GetProperties(rpc)
if (NOT rpc_POPULATED)
    FetchContent_Populate(rpc)
    add_subdirectory(${rpc_SOURCE_DIR} ${rpc_BINARY_DIR})
endif()

include_directories(${rpc_SOURCE_DIR}/include/)


FetchContent_Declare(
        spdlog
        GIT_REPOSITORY https://github.com/gabime/spdlog.git
        GIT_TAG        v1.x
)

FetchContent_GetProperties(spdlog)
if (NOT spdlog_POPULATED)
    FetchContent_Populate(spdlog)
    add_subdirectory(${spdlog_SOURCE_DIR} ${spdlog_BINARY_DIR})
endif()

include_directories(${spdlog_SOURCE_DIR}/include/)

enable_testing()


add_executable(
  socket_test
  socket_test.cpp
)

target_include_directories(socket_test PUBLIC ../include)

target_link_libraries(
  socket_test
  saba_socket
  gtest_main
)

add_executable(
  scheduler_test
  scheduler_test.cpp
)

target_include_directories(scheduler_test PUBLIC ../include)

target_link_libraries(
  scheduler_test
  scheduler
  gtest_main
)

include(GoogleTest)
gtest_discover_tests(socket_test)
gtest_discover_tests(scheduler_test)

source_group(
  TREE "${PROJECT_SOURCE_DIR}/include"
  PREFIX "Header Files"
  FILES ${HEADER_LIST})


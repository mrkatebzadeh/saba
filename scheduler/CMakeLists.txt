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

include(FetchContent)

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

include_directories(${spdlog_SOURCE_DIR}/include/)

include_directories(${rpc_SOURCE_DIR}/include/)

add_library(scheduler rpc.cpp scheduler.cpp hierarchical_cluster.cpp scheduler_args.cpp)

add_executable(saba_scheduler main.cpp)

target_link_libraries(saba_scheduler scheduler rpc spdlog::spdlog)

/**
 * Controller/main.cpp
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
#include <fstream>
#include <numeric>
#include <sstream>

#include "controller.h"
#include "controller_args.h"

int main(int argc, char **argv) {
    Controller controller;
    spdlog::info("Controller started.");

    auto config = parseOpts(argc, argv);

    rpc::server rpc_server(config.port);

    controller.available_pls = config.available_pls;
    controller.available_qs  = config.available_qs;

    if (config.algorithm == "ib") {
        controller.algorithm = AllocationAlgorithm::IB;
    } else if (config.algorithm == "idealmaxmin") {
        controller.algorithm = AllocationAlgorithm::IDEALMAXMIN;
    } else if (config.algorithm == "bestfitsmart") {
        controller.algorithm = AllocationAlgorithm::BESTFITSMART;
    } else if (config.algorithm == "hierarchicalsmart") {
        controller.algorithm = AllocationAlgorithm::HIERARCHICALSMART;
    } else if (config.algorithm == "idealsmart") {
        controller.algorithm = AllocationAlgorithm::IDEALSMART;
    } else {
        controller.algorithm = AllocationAlgorithm::IB;
    }

    spdlog::info(
        "Loading profile table from {} ...", config.profile_table_file);
    controller.loadProfileTable(config.profile_table_file);

    spdlog::info("Generating slowdoan table...");
    controller.generateSlowdownTable();

    spdlog::info("Generating sensitivity table...");
    controller.generateSensitivityTable();

    spdlog::info("Clustering applications...");
    controller.clusterApplications();

    spdlog::info("Clustering SLs...");
    controller.clusterPriorityLevels();

    spdlog::info("Serving ...");
    rpc_server.bind("app_register", [&controller](std::string application_name) {
        return appRegisterHandler(controller, application_name);
    });

    rpc_server.bind("connection_create", [&controller](std::string src, std::string dst,
                                             std::string application_name) {
        return connectionCreateHandler(controller, src, dst, application_name);
    });

    rpc_server.bind("connection_destroy", [&controller](int connection_fd) {
        connectionDestroyHandler(controller, connection_fd);
    });

    rpc_server.run();
    return 0;
}

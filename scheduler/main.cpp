/**
 * scheduler/main.cpp
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
#include "scheduler.h"
#include "scheduler_args.h"
#include <fstream>
#include <numeric>
#include <sstream>

Scheduler scheduler;

int main(int argc, char **argv) {

  spdlog::info("Scheduler started.");

  auto config = parse_opt(argc, argv);

  rpc::server rpc_server(config.port);

  scheduler.available_SLs = config.available_SLs;
  scheduler.available_VLs = config.available_VLs;

  if (config.algorithm == "ib") {
    scheduler.algorithm = AllocationAlgorithm::IB;
  } else if (config.algorithm == "idealmaxmin") {
    scheduler.algorithm = AllocationAlgorithm::IDEALMAXMIN;
  } else if (config.algorithm == "bestfitsmart") {
    scheduler.algorithm = AllocationAlgorithm::BESTFITSMART;
  } else if (config.algorithm == "hierarchicalsmart") {
    scheduler.algorithm = AllocationAlgorithm::HIERARCHICALSMART;
  } else if (config.algorithm == "idealsmart") {
    scheduler.algorithm = AllocationAlgorithm::IDEALSMART;
  } else {
    scheduler.algorithm = AllocationAlgorithm::IB;
  }

  spdlog::info("Loading profile table from {} ...", config.profile_table_file);
  scheduler.load_profile_table(config.profile_table_file);

  spdlog::info("Generating slowdoan table...");
  scheduler.generate_slowdown_table();

  spdlog::info("Generating sensitivity table...");
  scheduler.generate_sensitivity_table();

  spdlog::info("Clustering applications...");
  scheduler.cluster_applications();

  spdlog::info("Clustering SLs...");
  scheduler.cluster_SLs();

  spdlog::info("Serving ...");
  rpc_server.bind("app_register", [](std::string application_name) {
    return app_register_handler(&scheduler, application_name);
  });

  rpc_server.bind("connection_create", [](std::string src, std::string dst,
                                          std::string application_name) {
    return connection_create_handler(&scheduler, src, dst, application_name);
  });

  rpc_server.bind("connection_destroy", [](int connection_fd) {
    connection_destroy_handler(&scheduler, connection_fd);
  });

  rpc_server.run();
  return 0;
}

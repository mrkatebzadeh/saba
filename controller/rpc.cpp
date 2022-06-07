/**
 * Controller/rpc.cpp
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

#include "controller.h"

int appRegisterHandler(Controller *controller, std::string application_name) {
  int sl = 0;
  auto application_fd = controller->name_to_app_table[application_name];

  switch (controller->algorithm) {
    case AllocationAlgorithm::IB:
      sl = controller->calculatePriorityLevelsByMaxMin(application_fd);
      break;
    case AllocationAlgorithm::IDEALMAXMIN:
      sl = controller->calculatePriorityLevelsByIdealMaxMin(application_fd);
      break;
    case AllocationAlgorithm::IDEALSMART:
      sl = controller->calculatePriorityLevelsByIdealSmart(application_fd);
      break;
    case AllocationAlgorithm::BESTFITSMART:
      sl = controller->calculatePriorityLevelsByBestFitSmart(application_fd);
      break;
    case AllocationAlgorithm::HIERARCHICALSMART:
      sl = controller->calculatePriorityLevelsByHierarchicalSmart(
          application_fd);
      break;
    default:
      spdlog::error("Unknown allocation algorithm");
      break;
  }
  spdlog::info("Application: {} got SL: {}", application_fd, sl);
  return sl;
}

int connectionCreateHandler(Controller *controller, std::string src,
                            std::string dst, std::string application) {
  // TODO
}
void connectionDestroyHandler(Controller *controller, int connection_fd) {
  // TODO
}

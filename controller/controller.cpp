/**
 * Controller/Controller.cpp
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

#include <fstream>
#include <numeric>
#include <sstream>

#include "controller_args.h"

void Controller::loadProfileTable(std::string profile_table_file_address) {
  std::ifstream table_file(profile_table_file_address);
  std::string line, colname;
  double val;

  if (table_file.good()) {
    // Extract the first line in the file
    std::getline(table_file, line);
  }

  // Read data, line by line
  while (std::getline(table_file, line)) {
    // Create a stringstream of the current line
    ProfileRecord pr;
    std::stringstream ss(line);

    ss >> val;
    pr.app = (int)val;
    if (ss.peek() == ',') ss.ignore();
    ss >> val;
    pr.bw = (int)val;
    if (ss.peek() == ',') ss.ignore();
    ss >> val;
    pr.time = val;
    if (ss.peek() == ',') ss.ignore();
    ss >> val;
    pr.slowdown = val;

    spdlog::debug("loadProfileTable: Adding {}", pr);

    profile_table.push_back(pr);
  }

  table_file.close();
}

void Controller::generateSlowdownTable() {
  for (auto record : profile_table) {
    slowdown_table[record.app].push_back(record.slowdown);
  }
}

void Controller::generateSensitivityTable() {
  // For now, just simple average. //TODO weighted average
  for (auto app : slowdown_table) {
    sensitivity_table[app.first] =
        std::accumulate(app.second.begin(), app.second.end(), 0.0) /
        app.second.size();
  }
}

void Controller::clusterApplications() {
  int npoints = sensitivity_table.size();

  if (npoints == 0) {
    return;
  }

  int opt_method = HCLUST_METHOD_SINGLE;

  double *distmat = new double[(npoints * (npoints - 1)) / 2];
  int k = 0;

  for (int i = 0; i < npoints; i++) {
    for (int j = i + 1; j < npoints; j++) {
      distmat[k] = std::fabs(sensitivity_table[i] - sensitivity_table[j]);
      k++;
    }
  }

  // clustering call
  int *merge = new int[2 * (npoints - 1)];
  double *height = new double[npoints - 1];
  hclust_fast(npoints, distmat, opt_method, merge, height);

  int *labels = new int[npoints];
  cutree_k(npoints, merge, available_pls, labels);

  for (int i = 0; i < npoints; i++) {
    pl_to_app_table[labels[i]].push_back(i);
    app_to_pl_table.push_back(labels[i]);
  }

  // clean up
  delete[] distmat;
  delete[] merge;
  delete[] height;
  delete[] labels;
}

void Controller::clusterPriorityLevels() {
  int npoints = available_pls;

  if (npoints == 0) {
    return;
  }

  int opt_method = HCLUST_METHOD_SINGLE;

  double *distmat = new double[(npoints * (npoints - 1)) / 2];
  int k = 0;

  for (int i = 0; i < npoints; i++) {
    double sl1 = std::accumulate(pl_to_app_table[i].begin(),
                                 pl_to_app_table[i].end(), 0.0) /
                 pl_to_app_table[i].size();
    for (int j = i + 1; j < npoints; j++) {
      double sl2 = std::accumulate(pl_to_app_table[j].begin(),
                                   pl_to_app_table[j].end(), 0.0) /
                   pl_to_app_table[j].size();
      distmat[k] = std::fabs(sl1 - sl2);
      k++;
    }
  }

  // clustering call
  int *merge = new int[2 * (npoints - 1)];
  double *height = new double[npoints - 1];
  hclust_fast(npoints, distmat, opt_method, merge, height);

  int *labels = new int[npoints];
  cutree_k(npoints, merge, available_qs, labels);

  for (int i = 0; i < npoints; i++) {
    pl_to_q_table.push_back(labels[i]);
  }

  // clean up
  delete[] distmat;
  delete[] merge;
  delete[] height;
  delete[] labels;
}

int Controller::calculatePriorityLevelsByMaxMin(uint32_t application_fd) {
  return 1;
}

int Controller::calculatePriorityLevelsByIdealMaxMin(uint32_t application_fd) {
  return 0;  // TODO
}

int Controller::calculatePriorityLevelsByBestFitSmart(uint32_t application_fd) {
  return 0;  // TODO
}

int Controller::calculatePriorityLevelsByHierarchicalSmart(
    uint32_t application_fd) {
  return app_to_pl_table[application_fd];
}

int Controller::calculatePriorityLevelsByIdealSmart(uint32_t application_fd) {
  return 0;  // TODO
}

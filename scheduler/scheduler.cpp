/**
 * scheduler/scheduler.cpp
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

  auto config = parse_opt(argc, argv);
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

  scheduler.load_profile_table(config.profile_table_file);
  scheduler.generate_slowdown_table();
  scheduler.generate_sensitivity_table();
  scheduler.cluster_applications();
  scheduler.cluster_SLs();

  return 0;
}

void Scheduler::load_profile_table(std::string profile_table_file) {
  std::ifstream myFile(profile_table_file);
  std::string line, colname;
  double val;

  if (myFile.good()) {
    // Extract the first line in the file
    std::getline(myFile, line);
  }

  // Read data, line by line
  while (std::getline(myFile, line)) {
    // Create a stringstream of the current line
    ProfileRecord pr;
    std::stringstream ss(line);

    ss >> val;
    pr.app = (int)val;
    if (ss.peek() == ',')
      ss.ignore();
    ss >> val;
    pr.bw = (int)val;
    if (ss.peek() == ',')
      ss.ignore();
    ss >> val;
    pr.time = val;
    if (ss.peek() == ',')
      ss.ignore();
    ss >> val;
    pr.slowdown = val;
    profile_table.push_back(pr);
  }

  myFile.close();
}

void Scheduler::generate_slowdown_table() {
  for (auto record : profile_table) {
    slowdown_table[record.app].push_back(record.slowdown);
  }
}

void Scheduler::generate_sensitivity_table() {
  // For now, just simple average. //TODO weighted average
  for (auto app : slowdown_table) {
    sensitivity_table[app.first] =
        std::accumulate(app.second.begin(), app.second.end(), 0.0) /
        app.second.size();
  }
}

void Scheduler::cluster_applications() {
  int npoints = sensitivity_table.size();
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
  cutree_k(npoints, merge, available_SLs, labels);

  for (int i = 0; i < npoints; i++) {
    sl_to_app_table[labels[i]].push_back(i);
    app_to_sl_table.push_back(labels[i]);
  }

  // clean up
  delete[] distmat;
  delete[] merge;
  delete[] height;
  delete[] labels;
}

void Scheduler::cluster_SLs() {
  int npoints = available_SLs;
  int opt_method = HCLUST_METHOD_SINGLE;

  double *distmat = new double[(npoints * (npoints - 1)) / 2];
  int k = 0;

  for (int i = 0; i < npoints; i++) {
    double sl1 = std::accumulate(sl_to_app_table[i].begin(),
                                 sl_to_app_table[i].end(), 0.0) /
                 sl_to_app_table[i].size();
    for (int j = i + 1; j < npoints; j++) {
      double sl2 = std::accumulate(sl_to_app_table[j].begin(),
                                   sl_to_app_table[j].end(), 0.0) /
                   sl_to_app_table[j].size();
      distmat[k] = std::fabs(sl1 - sl2);
      k++;
    }
  }

  // clustering call
  int *merge = new int[2 * (npoints - 1)];
  double *height = new double[npoints - 1];
  hclust_fast(npoints, distmat, opt_method, merge, height);

  int *labels = new int[npoints];
  cutree_k(npoints, merge, available_VLs, labels);

  for (int i = 0; i < npoints; i++) {
    sl_to_vl_table.push_back(labels[i]);
  }

  // clean up
  delete[] distmat;
  delete[] merge;
  delete[] height;
  delete[] labels;
}

int Scheduler::calculate_SL_by_IB(std::string *c_msg) { return 1; }

int Scheduler::calculate_SL_by_idealmaxmin(std::string *c_msg) {

  return 0; // TODO
}

int Scheduler::calculate_SL_by_bestfitsmart(std::string *c_msg) {

  return 0; // TODO
}

int Scheduler::calculate_SL_by_hierarchicalsmart(std::string *c_msg) {
  int srcLid = 0; // TODO c_msg->getSrcLid();

  return app_to_sl_table[srcLid];
}

int Scheduler::calculate_SL_by_idealsmart(std::string *c_msg) {

  return 0; // TODO
}

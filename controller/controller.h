/**
 * Controller/Controller.h
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
 * OUT OF OR IN application WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
 * THE SOFTWARE.
 */

#ifndef CONTROLLER_H
#define CONTROLLER_H
#include <math.h>
#include <nlopt.h>
#include <rpc/server.h>
#include <spdlog/spdlog.h>
#include <spdlog/fmt/ostr.h>
#include <cstdint>
#include <map>
#include <vector>

#include "hierarchical_cluster.h"

enum class TargetType { TARGET_HCA = 0, TARGET_SWITCH = 1 };

enum class AllocationAlgorithm {
  IB,
  IDEALMAXMIN,
  BESTFITSMART,
  HIERARCHICALSMART,
  IDEALSMART
};

class ProfileRecord {
 public:
  std::uint32_t app;
  std::uint32_t bw;
  double time;
  double slowdown;
  friend std::ostream& operator<<(std::ostream& os, const ProfileRecord& obj);
};

std::ostream& operator<<(std::ostream& os, const ProfileRecord& obj) {
  os << obj.app << ',' << obj.bw << ',' << obj.slowdown << ',' << obj.time;
  return os;
}

class BandwidthAllocationRecord {
 public:
  std::uint32_t app;
  std::uint32_t src;
  std::uint32_t dst;
  std::uint32_t pl;
  std::uint32_t bw;
  friend std::ostream& operator<<(std::ostream& os,
                                  const BandwidthAllocationRecord& obj);
};

std::ostream& operator<<(std::ostream& os,
                         const BandwidthAllocationRecord& obj) {
  os << obj.app << ',' << obj.bw << ',' << obj.src << ',' << obj.dst << ','
     << obj.pl;
  return os;
}

class Connection {
 public:
  std::string src;
  std::string dst;
  std::string application;
  friend std::ostream& operator<<(std::ostream& os, const Connection& obj);
};

std::ostream& operator<<(std::ostream& os, const Connection& obj) {
  os << obj.src << ',' << obj.dst << ',' << obj.application;
  return os;
}

class IBSwitch {
 public:
  int id;
  std::string high_config;
  std::string low_config;
  std::map<int, Connection> connections;
  friend std::ostream& operator<<(std::ostream& os, const IBSwitch& obj);
};

std::ostream& operator<<(std::ostream& os, const IBSwitch& obj) {
  os << obj.id << ',' << obj.high_config << ',' << obj.low_config << '[';
  for (const auto& keyvalue : obj.connections) {
    os << keyvalue.second << ',';
  }
  os << ']';
  return os;
}

class Controller {
 public:
  // parameters:
  AllocationAlgorithm algorithm;
  int available_pls;
  int available_qs;
  std::vector<ProfileRecord> profile_table;
  std::vector<BandwidthAllocationRecord> allocation_table;

  // tables
  std::map<int, std::vector<double>> slowdown_table;
  std::map<int, double> sensitivity_table;
  std::map<int, std::vector<int>> pl_to_app_table;
  std::map<std::string, int> name_to_app_table;
  std::vector<int> conn_to_app_table;
  std::vector<int> app_to_pl_table;
  std::vector<int> pl_to_q_table;
  std::map<int, Connection> connections;
  std::map<int, IBSwitch> switch_configs;

  // methods
  void loadProfileTable(std::string profile_table_file);
  void generateSlowdownTable();
  void generateSensitivityTable();
  void clusterApplications();
  void clusterPriorityLevels();
  int calculatePriorityLevelsByMaxMin(uint32_t application_fd);
  int calculatePriorityLevelsByIdealMaxMin(uint32_t application_fd);
  int calculatePriorityLevelsByBestFitSmart(uint32_t application_fd);
  int calculatePriorityLevelsByHierarchicalSmart(uint32_t application_fd);
  int calculatePriorityLevelsByIdealSmart(uint32_t application_fd);
  void configAllSwitches();
  std::vector<IBSwitch> getPathSwitches(Connection connection);
};

int appRegisterHandler(Controller* controller, std::string application_name);
int connectionCreateHandler(Controller* controller, std::string src,
                            std::string dst, std::string application);
void connectionDestroyHandler(Controller* controller, int connection_fd);

#endif

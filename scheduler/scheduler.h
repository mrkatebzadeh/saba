/**
 * scheduler/scheduler.h
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

#ifndef SCHEDULER_H
#define SCHEDULER_H
#include "hirarchical_cluster.h"
#include <cstdint>
#include <map>
#include <vector>

class TargetType {
public:
  enum { TARGET_HCA = 0, TARGET_SWITCH = 1 };
};

class AllocationAlgorithm {
public:
  enum { IB, IDEALMAXMIN, BESTFITSMART, HIERARCHICALSMART, IDEALSMART };
};

class ProfileRecord {
public:
  std::uint32_t app;
  std::uint32_t bw;
  double time;
  double slowdown;
};

class BandwidthAllocationRecord {
public:
  std::uint32_t app;
  std::uint32_t src;
  std::uint32_t dst;
  std::uint32_t sl;
  std::uint32_t bw;
};

class Scheduler {
public:
  // parameters:
  AllocationAlgorithm algorithm;
  int available_SLs;
  int available_VLs;
  std::vector<ProfileRecord> profile_table;
  std::vector<BandwidthAllocationRecord> allocation_table;

  // tables
  std::map<int, std::vector<double> > slowdown_table;
  std::map<int, double> sensitivity_table;
  std::map<int, std::vector<int> > sl_to_app_table;
  std::vector<int> app_to_sl_table;
  std::vector<int> sl_to_vl_table;

  // methods
  void load_profile_table(const char *profile_table_file);
  void generate_slowdown_table();
  void generate_sensitivity_table();
  void cluster_applications();
  void cluster_SLs();
};

#endif

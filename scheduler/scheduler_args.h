/**
 * workload/args.h
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

#ifndef ARGS_H
#define ARGS_H
#include <getopt.h>
#include <iostream>
#include <string>

#define IBMAXSLVL 8

class SchedulerConfig {
public:
  bool verbose;
  int port;
  int available_SLs;
  int available_VLs;
  std::string algorithm;
  std::string profile_table_file;

  friend std::ostream &operator<<(std::ostream &strm, SchedulerConfig const& a) {
    return strm << "Config( SLs:" << a.available_SLs
                << ", VLs: " << a.available_VLs
                << ", algorithm: " << a.algorithm << " )";
  }
};

SchedulerConfig parse_opt(int argc, char **argv);

#endif

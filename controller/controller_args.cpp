/**
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

#include "controller_args.h"

#include "controller.h"

ControllerConfig parseOpts(int argc, char** argv) {
  int c;
  auto config               = ControllerConfig();
  config.algorithm          = std::string("hierarchicalsmart");
  config.port               = 8585;
  config.verbose            = true;
  config.available_pls      = IB_MAX_SLVL;
  config.available_qs       = IB_MAX_SLVL;
  config.profile_table_file = std::string("./profile_table.csv");

  while (1) {
    static struct option long_options[] = {

        {"verbose", no_argument, 0, 'v'},
        {"port", required_argument, 0, 'p'},
        {"algorithm", required_argument, 0, 'a'},
        {"table", required_argument, 0, 't'},
        {"vls", required_argument, 0, 'V'},
        {"sls", required_argument, 0, 'S'}};

    int option_index = 0;

    c = getopt_long(argc, argv, "vp:a:t:V:S:", long_options, &option_index);

    if (c == -1)
      break;

    switch (c) {
      case 0:
        if (long_options[option_index].flag != 0)
          break;
        std::printf("option %s", long_options[option_index].name);
        if (optarg)
          std::printf(" with arg %s", optarg);
        std::printf("\n");
        break;

      case 'v':
        config.verbose = true;
        break;

      case 'p':
        config.port = atoi(optarg);
        break;

      case 'a':
        config.algorithm = std::string(optarg);
        break;

      case 't':
        config.profile_table_file = std::string(optarg);
        break;

      case 'V':
        config.available_qs = atoi(optarg);
        break;

      case 'S':
        config.available_pls = atoi(optarg);
        break;

      default:
        printf("%c\n", c);
    }
  }

  return config;
}

/**
 * Controller/hierarchical_cluster.h
 * Copyright (c) 2021 M.R. Siavash Katebzadeh <mr.katebzadeh@gmail.com>
 * Copyright (c) 2011  Daniel Müllner
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

#ifndef __HC_H
#define __HC_H

//
// Assigns cluster labels (0, ..., nclust-1) to the n points such
// that the cluster result is split into nclust clusters.
//
// Input arguments:
//   n      = number of observables
//   merge  = clustering result in R format
//   nclust = number of clusters
// Output arguments:
//   labels = allocated integer array of size n for result
//
void cutree_k(int n, const int* merge, int nclust, int* labels);

//
// Assigns cluster labels (0, ..., nclust-1) to the n points such
// that the hierarchical clsutering is stopped at cluster distance cdist
//
// Input arguments:
//   n      = number of observables
//   merge  = clustering result in R format
//   height = cluster distance at each merge step
//   cdist  = cutoff cluster distance
// Output arguments:
//   labels = allocated integer array of size n for result
//
void cutree_cdist(
    int n, const int* merge, double* height, double cdist, int* labels);

//
// Hierarchical clustering with one of Daniel Muellner's fast algorithms
//
// Input arguments:
//   n       = number of observables
//   distmat = condensed distance matrix, i.e. an n*(n-1)/2 array representing
//             the upper triangle (without diagonal elements) of the distance
//             matrix, e.g. for n=4:
//               d00 d01 d02 d03
//               d10 d11 d12 d13   ->  d01 d02 d03 d12 d13 d23
//               d20 d21 d22 d23
//               d30 d31 d32 d33
//   method  = cluster metric (see enum hclust_fast_methods)
// Output arguments:
//   merge   = allocated (n-1)x2 matrix (2*(n-1) array) for storing result.
//             Result follows R hclust convention:
//              - observabe indices start with one
//              - merge[i][] contains the merged nodes in step i
//              - merge[i][j] is negative when the node is an atom
//   height  = allocated (n-1) array with distances at each merge step
// Return code:
//   0 = ok
//   1 = invalid method
//
int hclust_fast(int n, double* distmat, int method, int* merge, double* height);

enum hclust_fast_methods {
    // single link with the minimum spanning tree algorithm (Rohlf, 1973)
    HCLUST_METHOD_SINGLE = 0,
    // complete link with the nearest-neighbor-chain algorithm (Murtagh, 1984)
    HCLUST_METHOD_COMPLETE = 1,
    // unweighted average link with the nearest-neighbor-chain algorithm
    // (Murtagh, 1984)
    HCLUST_METHOD_AVERAGE = 2,
    // median link with the generic algorithm (Müllner, 2011)
    // requires euclidean distances as distance data
    HCLUST_METHOD_MEDIAN = 3
};

#endif


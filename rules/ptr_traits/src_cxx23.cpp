#include <algorithm>
#include <utility>

template <typename T1> auto f1(const T1 &a0) { return std::to_address<T1>(a0); }

template <typename T1> T1 *f2(T1 *a0) { return std::to_address<T1>(a0); }

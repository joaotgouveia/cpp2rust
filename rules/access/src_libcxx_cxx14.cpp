#include <iterator>
#include <utility>

template <typename T1> auto f7(const T1 &a0) { return std::cbegin<T1>(a0); }

template <typename T1> auto f8(const T1 &a0) { return std::cend<T1>(a0); }

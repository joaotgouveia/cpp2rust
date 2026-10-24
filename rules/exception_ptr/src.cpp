#include <algorithm>
#include <cstddef>
#include <exception>
#include <utility>

template <typename T1> std::exception_ptr f1(T1 a0) {
  return std::make_exception_ptr<T1>(std::move(a0));
}

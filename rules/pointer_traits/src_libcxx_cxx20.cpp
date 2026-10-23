#include <memory>
#include <utility>

template <typename T1> auto f4(const T1 &a0) { return std::to_address<T1>(a0); }

template <typename T1> auto f5(T1 *a0) { return std::to_address<T1>(a0); }

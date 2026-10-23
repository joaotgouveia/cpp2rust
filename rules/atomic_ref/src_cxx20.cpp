#include <atomic>
#include <cstddef>
#include <utility>

template <typename T1> using t1 = std::atomic_ref<T1>;

template <typename T1> std::atomic_ref<T1> f1(T1 &a0) {
  return std::atomic_ref<T1>(a0);
}

template <typename T1> std::atomic_ref<T1> f2(const std::atomic_ref<T1> &a0) {
  return std::atomic_ref<T1>(a0);
}

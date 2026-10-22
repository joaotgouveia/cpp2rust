#include "../common/init.h"
#include "../common/traits.h"
#include <atomic>
#include <cstddef>
#include <utility>

template <typename T1>
void f399(volatile std::atomic<T1> *a0,
          typename std::atomic<T1>::value_type a1) {
  return std::atomic_init<T1>(a0, a1);
}

template <typename T1>
void f400(std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1) {
  return std::atomic_init<T1>(a0, a1);
}

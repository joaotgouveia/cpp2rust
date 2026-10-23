#include <condition_variable>
#include <mutex>
#include <utility>

template <typename T1> std::lock_guard<T1> f2(T1 &a0) {
  return std::lock_guard<T1>(a0);
}

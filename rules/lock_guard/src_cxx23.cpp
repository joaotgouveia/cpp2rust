#include <atomic>
#include <utility>

template <typename T1> using t1 = std::lock_guard<T1>;

template <typename T1> using t2 = typename std::lock_guard<T1>::mutex_type;

template <typename T1> std::lock_guard<T1> f1(T1 &a0) {
  return std::lock_guard<T1>(a0);
}

template <typename T1> std::lock_guard<T1> f2(T1 &a0, std::adopt_lock_t a1) {
  return {a0, std::move(a1)};
}

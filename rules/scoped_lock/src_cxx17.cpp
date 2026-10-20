#include "../common/init.h"
#include "../common/traits.h"
#include <condition_variable>
#include <mutex>
#include <utility>

template <typename T1> using t1 = std::scoped_lock<T1>;

template <typename T1> using t2 = typename std::scoped_lock<T1>::mutex_type;

template <typename T1>
std::scoped_lock<T1> f1(typename std::scoped_lock<T1>::mutex_type &a0) {
  return std::scoped_lock<T1>(a0);
}

template <typename T1>
std::scoped_lock<T1> f2(std::adopt_lock_t a0,
                        typename std::scoped_lock<T1>::mutex_type &a1) {
  return std::scoped_lock<T1>(std::move(a0), a1);
}

template <typename... T1> using t3 = std::scoped_lock<T1...>;

template <typename... T1> std::scoped_lock<T1...> f3(T1 &...a0) {
  return std::scoped_lock<T1...>(a0...);
}

template <typename... T1>
std::scoped_lock<T1...> f4(std::adopt_lock_t a0, T1 &...a1) {
  return std::scoped_lock<T1...>(std::move(a0), a1...);
}

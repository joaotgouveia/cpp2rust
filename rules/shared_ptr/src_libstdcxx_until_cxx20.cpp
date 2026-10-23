#include <algorithm>
#include <condition_variable>
#include <utility>

template <typename T1, typename T2, typename... T3>
std::shared_ptr<typename std::enable_if<!std::is_array<T1>::value, T1>::type>
f45(const T2 &a0, T3 &&...a1) {
  return std::allocate_shared<T1, T2, T3...>(a0, std::forward<T3>(a1)...);
}

template <typename T1, typename... T2>
std::shared_ptr<typename std::enable_if<!std::is_array<T1>::value, T1>::type>
f46(T2 &&...a0) {
  return std::make_shared<T1, T2...>(std::forward<T2>(a0)...);
}

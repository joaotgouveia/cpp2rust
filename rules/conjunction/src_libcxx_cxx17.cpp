#include <type_traits>
#include <utility>

template <typename... T1>
typename std::conjunction<T1...>::value_type
f1(const std::conjunction<T1...> &a0) {
  return static_cast<typename std::conjunction<T1...>::value_type>(a0);
}

template <typename... T1>
typename std::conjunction<T1...>::value_type
f2(const std::conjunction<T1...> &a0) {
  return a0.operator()();
}

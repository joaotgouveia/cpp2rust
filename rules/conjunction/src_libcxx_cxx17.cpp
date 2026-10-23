#include <type_traits>
#include <utility>

template <typename T1> using t3 = std::conjunction<T1>;

template <typename T1, typename... T2> using t4 = std::conjunction<T1, T2...>;

template <typename... T1> bool f8(const std::conjunction<T1...> &a0) {
  return static_cast<bool>(a0);
}

template <typename... T1> bool f9(const std::conjunction<T1...> &a0) {
  return a0.operator()();
}

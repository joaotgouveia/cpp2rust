#include <type_traits>
#include <utility>

template <
    typename T1, typename... T2,
    typename std::enable_if<std::is_base_of<std::conjunction<T2...>,
                                            std::conjunction<T1, T2...>>::value,
                            int>::type T3 = 0>
bool f10(const std::conjunction<T1, T2...> &a0) {
  return static_cast<bool>(a0);
}

template <
    typename T1, typename... T2,
    typename std::enable_if<std::is_base_of<std::conjunction<T2...>,
                                            std::conjunction<T1, T2...>>::value,
                            int>::type T3 = 0>
bool f11(const std::conjunction<T1, T2...> &a0) {
  return a0.operator()();
}

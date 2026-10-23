#include <algorithm>
#include <cstddef>
#include <functional>
#include <memory>
#include <string>
#include <string_view>
#include <system_error>
#include <typeindex>
#include <utility>
#include <vector>

template <typename T1>
using t58 = typename std::hash<std::shared_ptr<T1>>::argument_type;

template <typename T1>
using t59 = typename std::hash<std::shared_ptr<T1>>::result_type;

template <typename T1, typename T2>
using t60 = typename std::hash<std::__enable_hash_helper<
    std::unique_ptr<T1, T2>,
    typename std::unique_ptr<T1, T2>::pointer>>::argument_type;

template <typename T1, typename T2>
using t61 = typename std::hash<std::__enable_hash_helper<
    std::unique_ptr<T1, T2>,
    typename std::unique_ptr<T1, T2>::pointer>>::result_type;

template <typename T1,
          typename std::enable_if<
              (std::is_same<void, typename std::enable_if<
                                      std::is_integral<T1>::value &&
                                          std::__is_unqualified_v<T1> &&
                                          (sizeof(T1) <= sizeof(std::size_t)),
                                      void>::type>::value),
              int>::type T2 = 0>
std::size_t f247(const std::hash<T1> &a0, T1 a1) {
  return a0.operator()(a1);
}

template <typename T1,
          typename std::enable_if<
              (std::is_same<
                  void, typename std::enable_if<std::is_enum<T1>::value &&
                                                    std::__is_unqualified_v<T1>,
                                                void>::type>::value),
              int>::type T2 = 0>
std::size_t f248(const std::hash<T1> &a0, T1 a1) {
  return a0.operator()(a1);
}

template <typename T1,
          typename std::enable_if<
              (std::is_same<void, typename std::enable_if<
                                      std::is_floating_point<T1>::value &&
                                          std::__is_unqualified_v<T1>,
                                      void>::type>::value),
              int>::type T2 = 0>
std::size_t f249(const std::hash<T1> &a0, T1 a1) {
  return a0.operator()(a1);
}

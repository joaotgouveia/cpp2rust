#include <algorithm>
#include <bitset>
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
using t60 = typename std::hash<std::shared_ptr<T1>>::argument_type;

template <typename T1>
using t61 = typename std::hash<std::shared_ptr<T1>>::result_type;

template <typename T1, typename T2>
using t62 = typename std::hash<std::__enable_hash_helper<
    std::unique_ptr<T1, T2>,
    typename std::unique_ptr<T1, T2>::pointer>>::argument_type;

template <typename T1, typename T2>
using t63 = typename std::hash<std::__enable_hash_helper<
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
std::size_t f271(const std::hash<T1> &a0, T1 a1) {
  return a0.operator()(std::move(a1));
}

template <typename T1,
          typename std::enable_if<
              (std::is_same<
                  void, typename std::enable_if<std::is_enum<T1>::value &&
                                                    std::__is_unqualified_v<T1>,
                                                void>::type>::value),
              int>::type T2 = 0>
std::size_t f272(const std::hash<T1> &a0, T1 a1) {
  return a0.operator()(std::move(a1));
}

template <typename T1,
          typename std::enable_if<(sizeof(T1) / sizeof(std::size_t) == 2UL),
                                  int>::type T2 = 0,
          typename std::enable_if<
              (std::is_same<void, typename std::enable_if<
                                      std::is_integral<T1>::value &&
                                          std::__is_unqualified_v<T1> &&
                                          (sizeof(T1) > sizeof(std::size_t)),
                                      void>::type>::value),
              int>::type T3 = 0>
std::size_t f273(const std::hash<T1> &a0, T1 a1) {
  return a0.operator()(std::move(a1));
}

template <typename T1,
          typename std::enable_if<(sizeof(T1) / sizeof(std::size_t) == 0UL),
                                  int>::type T2 = 0,
          typename std::enable_if<
              (std::is_same<void, typename std::enable_if<
                                      std::is_integral<T1>::value &&
                                          std::__is_unqualified_v<T1> &&
                                          (sizeof(T1) > sizeof(std::size_t)),
                                      void>::type>::value),
              int>::type T3 = 0>
std::size_t f274(const std::hash<T1> &a0, T1 a1) {
  return a0.operator()(std::move(a1));
}

template <typename T1,
          typename std::enable_if<(sizeof(T1) / sizeof(std::size_t) == 1UL),
                                  int>::type T2 = 0,
          typename std::enable_if<
              (std::is_same<void, typename std::enable_if<
                                      std::is_integral<T1>::value &&
                                          std::__is_unqualified_v<T1> &&
                                          (sizeof(T1) > sizeof(std::size_t)),
                                      void>::type>::value),
              int>::type T3 = 0>
std::size_t f275(const std::hash<T1> &a0, T1 a1) {
  return a0.operator()(std::move(a1));
}

template <typename T1,
          typename std::enable_if<(sizeof(T1) / sizeof(std::size_t) == 3UL),
                                  int>::type T2 = 0,
          typename std::enable_if<
              (std::is_same<void, typename std::enable_if<
                                      std::is_integral<T1>::value &&
                                          std::__is_unqualified_v<T1> &&
                                          (sizeof(T1) > sizeof(std::size_t)),
                                      void>::type>::value),
              int>::type T3 = 0>
std::size_t f276(const std::hash<T1> &a0, T1 a1) {
  return a0.operator()(std::move(a1));
}

template <typename T1,
          typename std::enable_if<(sizeof(T1) / sizeof(std::size_t) == 4UL),
                                  int>::type T2 = 0,
          typename std::enable_if<
              (std::is_same<void, typename std::enable_if<
                                      std::is_integral<T1>::value &&
                                          std::__is_unqualified_v<T1> &&
                                          (sizeof(T1) > sizeof(std::size_t)),
                                      void>::type>::value),
              int>::type T3 = 0>
std::size_t f277(const std::hash<T1> &a0, T1 a1) {
  return a0.operator()(std::move(a1));
}

template <typename T1,
          typename std::enable_if<
              (std::is_same<void, typename std::enable_if<
                                      std::is_floating_point<T1>::value &&
                                          std::__is_unqualified_v<T1>,
                                      void>::type>::value),
              int>::type T2 = 0>
std::size_t f278(const std::hash<T1> &a0, T1 a1) {
  return a0.operator()(std::move(a1));
}

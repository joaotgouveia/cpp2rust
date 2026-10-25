#include <algorithm>
#include <bitset>
#include <condition_variable>
#include <future>
#include <string>
#include <system_error>
#include <typeindex>
#include <utility>
#include <vector>

template <typename T1, typename T2,
          typename std::enable_if<
              std::is_base_of<std::unique_ptr<T1, T2>,
                              std::hash<std::unique_ptr<T1, T2>>>::value,
              int>::type T3 = 0>
typename std::add_lvalue_reference<T1>::type
f154(const std::hash<std::unique_ptr<T1, T2>> &a0) {
  return a0.operator*();
}

template <typename T1, typename T2,
          typename std::enable_if<
              std::is_base_of<std::unique_ptr<T1, T2>,
                              std::hash<std::unique_ptr<T1, T2>>>::value,
              int>::type T3 = 0>
T1 *f155(const std::hash<std::unique_ptr<T1, T2>> &a0) {
  return a0.operator->();
}

template <typename T1, typename T2,
          typename std::enable_if<
              std::is_base_of<std::unique_ptr<T1, T2>,
                              std::hash<std::unique_ptr<T1, T2>>>::value,
              int>::type T3 = 0>
T1 *f156(const std::hash<std::unique_ptr<T1, T2>> &a0) {
  return a0.get();
}

template <typename T1, typename T2,
          typename std::enable_if<
              std::is_base_of<std::unique_ptr<T1, T2>,
                              std::hash<std::unique_ptr<T1, T2>>>::value,
              int>::type T3 = 0>
T2 &f157(std::hash<std::unique_ptr<T1, T2>> &a0) {
  return a0.get_deleter();
}

template <typename T1, typename T2,
          typename std::enable_if<
              std::is_base_of<std::unique_ptr<T1, T2>,
                              std::hash<std::unique_ptr<T1, T2>>>::value,
              int>::type T3 = 0>
T2 const &f158(const std::hash<std::unique_ptr<T1, T2>> &a0) {
  return a0.get_deleter();
}

template <typename T1, typename T2,
          typename std::enable_if<
              std::is_base_of<std::unique_ptr<T1, T2>,
                              std::hash<std::unique_ptr<T1, T2>>>::value,
              int>::type T3 = 0>
bool f159(const std::hash<std::unique_ptr<T1, T2>> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2,
          typename std::enable_if<
              std::is_base_of<std::unique_ptr<T1, T2>,
                              std::hash<std::unique_ptr<T1, T2>>>::value,
              int>::type T3 = 0>
T1 *f160(std::hash<std::unique_ptr<T1, T2>> &a0) {
  return a0.release();
}

template <typename T1, typename T2,
          typename std::enable_if<
              std::is_base_of<std::unique_ptr<T1, T2>,
                              std::hash<std::unique_ptr<T1, T2>>>::value,
              int>::type T3 = 0>
void f161(std::hash<std::unique_ptr<T1, T2>> &a0, T1 *a1) {
  return a0.reset(a1);
}

template <typename T1, typename T2, typename T3, typename T4,
          typename std::enable_if<(!(std::is_same<T3, T1>::value &&
                                     std::is_same<T4, T2>::value)),
                                  int>::type T5 = 0,
          typename std::enable_if<
              std::is_base_of<std::unique_ptr<T1, T2>,
                              std::hash<std::unique_ptr<T1, T2>>>::value,
              int>::type T6 = 0>
typename std::enable_if<
    std::__and_<std::__and_<std::is_convertible<
                                typename std::unique_ptr<T3, T4>::pointer,
                                typename std::__uniq_ptr_impl<T1, T2>::pointer>,
                            std::__not_<std::is_array<T3>>>,
                std::is_assignable<T2 &, T4 &&>>::value,
    std::unique_ptr<T1, T2> &>::type
f162(std::hash<std::unique_ptr<T1, T2>> &a0, std::unique_ptr<T3, T4> &&a1) {
  return a0.template operator= <T3, T4>(std::move(a1));
}

template <typename T1,
          typename std::enable_if<
              std::is_base_of<std::__hash_enum<T1>, std::hash<T1>>::value,
              int>::type T2 = 0>
std::size_t f163(const std::hash<T1> &a0, T1 a1) {
  return a0.operator()(std::move(a1));
}

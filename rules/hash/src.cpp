#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <bitset>
#include <condition_variable>
#include <cstddef>
#include <functional>
#include <future>
#include <memory>
#include <string>
#include <system_error>
#include <typeindex>
#include <utility>
#include <vector>

using t1 = std::hash<std::error_code>;

std::size_t f1(const std::hash<std::error_code> &a0,
               const std::error_code &a1) {
  return a0.operator()(a1);
}

std::hash<std::error_code> f2() { return std::hash<std::error_code>(); }

std::hash<std::error_code> f3(const std::hash<std::error_code> &a0) {
  return std::hash<std::error_code>(a0);
}

std::hash<std::error_code> f4(std::hash<std::error_code> &&a0) {
  return std::hash<std::error_code>(std::move(a0));
}

std::hash<std::error_code> &f5(std::hash<std::error_code> &a0,
                               const std::hash<std::error_code> &a1) {
  return a0.operator=(a1);
}

std::hash<std::error_code> &f6(std::hash<std::error_code> &a0,
                               std::hash<std::error_code> &&a1) {
  return a0.operator=(std::move(a1));
}

using t2 = std::hash<std::type_index>;

std::hash<std::type_index> f7() { return std::hash<std::type_index>(); }

std::hash<std::type_index> f8(const std::hash<std::type_index> &a0) {
  return std::hash<std::type_index>(a0);
}

std::hash<std::type_index> f9(std::hash<std::type_index> &&a0) {
  return std::hash<std::type_index>(std::move(a0));
}

std::hash<std::type_index> &f10(std::hash<std::type_index> &a0,
                                const std::hash<std::type_index> &a1) {
  return a0.operator=(a1);
}

std::hash<std::type_index> &f11(std::hash<std::type_index> &a0,
                                std::hash<std::type_index> &&a1) {
  return a0.operator=(std::move(a1));
}

template <std::size_t T1> using t3 = std::hash<std::bitset<T1>>;

template <typename T1> using t4 = std::hash<T1 *>;

template <typename T1> std::size_t f12(const std::hash<T1 *> &a0, T1 *a1) {
  return a0.operator()(a1);
}

template <typename T1> using t5 = std::hash<std::shared_ptr<T1>>;

template <typename T1>
std::size_t f13(const std::hash<std::shared_ptr<T1>> &a0,
                const std::shared_ptr<T1> &a1) {
  return a0.operator()(a1);
}

template <typename T1> using t6 = std::hash<std::vector<bool, T1>>;

template <typename T1>
std::size_t f14(const std::hash<std::vector<bool, T1>> &a0,
                const std::vector<bool, T1> &a1) {
  return a0.operator()(a1);
}

template <typename T1> using t7 = std::hash<T1>;

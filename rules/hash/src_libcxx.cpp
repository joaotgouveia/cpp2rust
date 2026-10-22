#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <cstddef>
#include <functional>
#include <memory>
#include <system_error>
#include <typeindex>
#include <utility>
#include <vector>

using t42 = std::hash<std::error_condition>;

std::size_t f204(const std::hash<std::error_condition> &a0,
                 const std::error_condition &a1) {
  return a0.operator()(a1);
}

std::hash<std::error_condition> f205() {
  return std::hash<std::error_condition>();
}

std::hash<std::error_condition>
f206(const std::hash<std::error_condition> &a0) {
  return std::hash<std::error_condition>(a0);
}

std::hash<std::error_condition> f207(std::hash<std::error_condition> &&a0) {
  return std::hash<std::error_condition>(std::move(a0));
}

std::hash<std::error_condition> &
f208(std::hash<std::error_condition> &a0,
     const std::hash<std::error_condition> &a1) {
  return a0.operator=(a1);
}

std::hash<std::error_condition> &f209(std::hash<std::error_condition> &a0,
                                      std::hash<std::error_condition> &&a1) {
  return a0.operator=(std::move(a1));
}

std::size_t f210(const std::hash<std::type_index> &a0, std::type_index a1) {
  return a0.operator()(std::move(a1));
}

using t43 = std::hash<std::nullptr_t>;

std::size_t f211(const std::hash<std::nullptr_t> &a0, std::nullptr_t a1) {
  return a0.operator()(a1);
}

std::hash<std::nullptr_t> f212() { return std::hash<std::nullptr_t>(); }

std::hash<std::nullptr_t> f213(const std::hash<std::nullptr_t> &a0) {
  return std::hash<std::nullptr_t>(a0);
}

std::hash<std::nullptr_t> f214(std::hash<std::nullptr_t> &&a0) {
  return std::hash<std::nullptr_t>(std::move(a0));
}

std::hash<std::nullptr_t> &f215(std::hash<std::nullptr_t> &a0,
                                const std::hash<std::nullptr_t> &a1) {
  return a0.operator=(a1);
}

std::hash<std::nullptr_t> &f216(std::hash<std::nullptr_t> &a0,
                                std::hash<std::nullptr_t> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
using t44 = std::hash<std::__enable_hash_helper<
    std::unique_ptr<T1, T2>, typename std::unique_ptr<T1, T2>::pointer>>;

template <typename T1, typename T2>
std::size_t
f217(const std::hash<std::__enable_hash_helper<
         std::unique_ptr<T1, T2>, typename std::unique_ptr<T1, T2>::pointer>>
         &a0,
     const std::unique_ptr<T1, T2> &a1) {
  return a0.operator()(a1);
}

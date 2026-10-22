#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <bitset>
#include <cstddef>
#include <functional>
#include <memory>
#include <system_error>
#include <typeindex>
#include <utility>
#include <vector>

template <typename T1>
using t46 = typename std::hash<std::shared_ptr<T1>>::argument_type;

template <typename T1>
using t47 = typename std::hash<std::shared_ptr<T1>>::result_type;

template <typename T1>
using t48 =
    typename std::hash<std::shared_ptr<T1>>::argument_type::element_type;

template <typename T1>
typename std::hash<std::shared_ptr<T1>>::argument_type f218() {
  return typename std::hash<std::shared_ptr<T1>>::argument_type();
}

template <typename T1>
typename std::hash<std::shared_ptr<T1>>::argument_type f219(std::nullptr_t a0) {
  return typename std::hash<std::shared_ptr<T1>>::argument_type(a0);
}

template <typename T1>
typename std::hash<std::shared_ptr<T1>>::argument_type
f220(const typename std::hash<std::shared_ptr<T1>>::argument_type &a0) {
  return typename std::hash<std::shared_ptr<T1>>::argument_type(a0);
}

template <typename T1>
typename std::hash<std::shared_ptr<T1>>::argument_type
f221(typename std::hash<std::shared_ptr<T1>>::argument_type &&a0) {
  return typename std::hash<std::shared_ptr<T1>>::argument_type(std::move(a0));
}

template <typename T1>
typename std::hash<std::shared_ptr<T1>>::argument_type &
f222(typename std::hash<std::shared_ptr<T1>>::argument_type &a0,
     const typename std::hash<std::shared_ptr<T1>>::argument_type &a1) {
  return a0.operator=(a1);
}

template <typename T1>
typename std::hash<std::shared_ptr<T1>>::argument_type &
f223(typename std::hash<std::shared_ptr<T1>>::argument_type &a0,
     typename std::hash<std::shared_ptr<T1>>::argument_type &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
void f224(typename std::hash<std::shared_ptr<T1>>::argument_type &a0,
          typename std::hash<std::shared_ptr<T1>>::argument_type &a1) {
  return a0.swap(a1);
}

template <typename T1>
void f225(typename std::hash<std::shared_ptr<T1>>::argument_type &a0) {
  return a0.reset();
}

template <typename T1>
typename std::hash<std::shared_ptr<T1>>::argument_type::element_type *
f226(const typename std::hash<std::shared_ptr<T1>>::argument_type &a0) {
  return a0.get();
}

template <typename T1>
__add_lvalue_reference(T1)
f227(const typename std::hash<std::shared_ptr<T1>>::argument_type &a0) {
  return a0.operator*();
}

template <typename T1>
typename std::hash<std::shared_ptr<T1>>::argument_type::element_type *
f228(const typename std::hash<std::shared_ptr<T1>>::argument_type &a0) {
  return a0.operator->();
}

template <typename T1>
long f229(const typename std::hash<std::shared_ptr<T1>>::argument_type &a0) {
  return a0.use_count();
}

template <typename T1>
bool f230(const typename std::hash<std::shared_ptr<T1>>::argument_type &a0) {
  return a0.unique();
}

template <typename T1>
bool f231(const typename std::hash<std::shared_ptr<T1>>::argument_type &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2>
typename std::hash<std::shared_ptr<T1>>::argument_type f232(T2 *a0) {
  return typename std::hash<std::shared_ptr<T1>>::argument_type(a0);
}

template <typename T1, typename T2, typename T3>
typename std::hash<std::shared_ptr<T1>>::argument_type f233(T2 *a0, T3 a1) {
  return typename std::hash<std::shared_ptr<T1>>::argument_type(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::hash<std::shared_ptr<T1>>::argument_type f234(T2 *a0, T3 a1,
                                                            T4 a2) {
  return typename std::hash<std::shared_ptr<T1>>::argument_type(a0, a1, a2);
}

template <typename T1, typename T2>
typename std::hash<std::shared_ptr<T1>>::argument_type
f235(const std::shared_ptr<T2> &a0,
     typename std::hash<std::shared_ptr<T1>>::argument_type::element_type *a1) {
  return typename std::hash<std::shared_ptr<T1>>::argument_type(a0, a1);
}

template <typename T1, typename T2>
typename std::hash<std::shared_ptr<T1>>::argument_type
f236(const std::shared_ptr<T2> &a0) {
  return typename std::hash<std::shared_ptr<T1>>::argument_type(a0);
}

template <typename T1, typename T2>
typename std::hash<std::shared_ptr<T1>>::argument_type
f237(std::shared_ptr<T2> &&a0) {
  return typename std::hash<std::shared_ptr<T1>>::argument_type(std::move(a0));
}

template <typename T1, typename T2>
typename std::hash<std::shared_ptr<T1>>::argument_type
f238(const std::weak_ptr<T2> &a0) {
  return typename std::hash<std::shared_ptr<T1>>::argument_type(a0);
}

template <typename T1, typename T2>
typename std::hash<std::shared_ptr<T1>>::argument_type
f239(std::auto_ptr<T2> &&a0) {
  return typename std::hash<std::shared_ptr<T1>>::argument_type(std::move(a0));
}

template <typename T1, typename T2, typename T3>
typename std::hash<std::shared_ptr<T1>>::argument_type
f240(std::unique_ptr<T2, T3> &&a0) {
  return typename std::hash<std::shared_ptr<T1>>::argument_type(std::move(a0));
}

template <typename T1, typename T2>
typename std::hash<std::shared_ptr<T1>>::argument_type &
f241(typename std::hash<std::shared_ptr<T1>>::argument_type &a0,
     const std::shared_ptr<T2> &a1) {
  return a0.template operator= <T2>(a1);
}

template <typename T1, typename T2>
typename std::hash<std::shared_ptr<T1>>::argument_type &
f242(typename std::hash<std::shared_ptr<T1>>::argument_type &a0,
     std::shared_ptr<T2> &&a1) {
  return a0.template operator= <T2>(std::move(a1));
}

template <typename T1, typename T2>
typename std::hash<std::shared_ptr<T1>>::argument_type &
f243(typename std::hash<std::shared_ptr<T1>>::argument_type &a0,
     std::auto_ptr<T2> &&a1) {
  return a0.template operator= <T2>(std::move(a1));
}

template <typename T1, typename T2, typename T3>
typename std::hash<std::shared_ptr<T1>>::argument_type &
f244(typename std::hash<std::shared_ptr<T1>>::argument_type &a0,
     std::unique_ptr<T2, T3> &&a1) {
  return a0.template operator= <T2, T3>(std::move(a1));
}

template <typename T1, typename T2>
void f245(typename std::hash<std::shared_ptr<T1>>::argument_type &a0, T2 *a1) {
  return a0.template reset<T2>(a1);
}

template <typename T1, typename T2, typename T3>
void f246(typename std::hash<std::shared_ptr<T1>>::argument_type &a0, T2 *a1,
          T3 a2) {
  return a0.template reset<T2, T3>(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
void f247(typename std::hash<std::shared_ptr<T1>>::argument_type &a0, T2 *a1,
          T3 a2, T4 a3) {
  return a0.template reset<T2, T3, T4>(a1, a2, a3);
}

template <typename T1, typename T2>
bool f248(const typename std::hash<std::shared_ptr<T1>>::argument_type &a0,
          const std::shared_ptr<T2> &a1) {
  return a0.template owner_before<T2>(a1);
}

template <typename T1, typename T2>
bool f249(const typename std::hash<std::shared_ptr<T1>>::argument_type &a0,
          const std::weak_ptr<T2> &a1) {
  return a0.template owner_before<T2>(a1);
}

template <typename T1, typename T2>
using t49 = typename std::hash<std::__enable_hash_helper<
    std::unique_ptr<T1, T2>,
    typename std::unique_ptr<T1, T2>::pointer>>::argument_type;

template <typename T1, typename T2>
using t50 = typename std::hash<std::__enable_hash_helper<
    std::unique_ptr<T1, T2>,
    typename std::unique_ptr<T1, T2>::pointer>>::result_type;

template <typename T1, typename T2>
using t51 = typename std::hash<std::__enable_hash_helper<
    std::unique_ptr<T1, T2>,
    typename std::unique_ptr<T1, T2>::pointer>>::argument_type::element_type;

template <typename T1, typename T2>
using t52 = typename std::hash<std::__enable_hash_helper<
    std::unique_ptr<T1, T2>,
    typename std::unique_ptr<T1, T2>::pointer>>::argument_type::deleter_type;

template <typename T1, typename T2>
using t53 = typename std::hash<std::__enable_hash_helper<
    std::unique_ptr<T1, T2>,
    typename std::unique_ptr<T1, T2>::pointer>>::argument_type::pointer;

template <typename T1, typename T2>
bool f250(const typename std::hash<std::__enable_hash_helper<
              std::unique_ptr<T1, T2>,
              typename std::unique_ptr<T1, T2>::pointer>>::argument_type &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2>
typename std::hash<std::__enable_hash_helper<
    std::unique_ptr<T1, T2>,
    typename std::unique_ptr<T1, T2>::pointer>>::argument_type
f251() {
  return typename std::hash<std::__enable_hash_helper<
      std::unique_ptr<T1, T2>,
      typename std::unique_ptr<T1, T2>::pointer>>::argument_type();
}

template <typename T1, typename T2>
typename std::hash<std::__enable_hash_helper<
    std::unique_ptr<T1, T2>,
    typename std::unique_ptr<T1, T2>::pointer>>::argument_type
f252(std::nullptr_t a0) {
  return typename std::hash<std::__enable_hash_helper<
      std::unique_ptr<T1, T2>,
      typename std::unique_ptr<T1, T2>::pointer>>::argument_type(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::hash<std::__enable_hash_helper<
    std::unique_ptr<T1, T2>,
    typename std::unique_ptr<T1, T2>::pointer>>::argument_type
f253(std::unique_ptr<T3, T4> &&a0) {
  return typename std::hash<std::__enable_hash_helper<
      std::unique_ptr<T1, T2>,
      typename std::unique_ptr<T1, T2>::pointer>>::argument_type(std::move(a0));
}

template <typename T1, typename T2, typename T3>
typename std::hash<std::__enable_hash_helper<
    std::unique_ptr<T1, T2>,
    typename std::unique_ptr<T1, T2>::pointer>>::argument_type
f254(std::auto_ptr<T3> &&a0) {
  return typename std::hash<std::__enable_hash_helper<
      std::unique_ptr<T1, T2>,
      typename std::unique_ptr<T1, T2>::pointer>>::argument_type(std::move(a0));
}

template <typename T1, typename T2>
__add_lvalue_reference(T1)
f255(const typename std::hash<std::__enable_hash_helper<
         std::unique_ptr<T1, T2>,
         typename std::unique_ptr<T1, T2>::pointer>>::argument_type &a0) {
  return a0.operator*();
}

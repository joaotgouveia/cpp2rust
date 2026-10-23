#include <algorithm>
#include <cstddef>
#include <functional>
#include <memory>
#include <system_error>
#include <typeindex>
#include <utility>
#include <vector>

template <typename T1>
using t45 = typename std::hash<std::shared_ptr<T1>>::argument_type;

template <typename T1>
using t46 = typename std::hash<std::shared_ptr<T1>>::result_type;

template <typename T1>
using t47 =
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
typename std::hash<std::shared_ptr<T1>>::argument_type::element_type *
f227(const typename std::hash<std::shared_ptr<T1>>::argument_type &a0) {
  return a0.operator->();
}

template <typename T1>
long f228(const typename std::hash<std::shared_ptr<T1>>::argument_type &a0) {
  return a0.use_count();
}

template <typename T1>
bool f229(const typename std::hash<std::shared_ptr<T1>>::argument_type &a0) {
  return a0.unique();
}

template <typename T1>
bool f230(const typename std::hash<std::shared_ptr<T1>>::argument_type &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2>
typename std::hash<std::shared_ptr<T1>>::argument_type
f231(const std::shared_ptr<T2> &a0,
     typename std::hash<std::shared_ptr<T1>>::argument_type::element_type *a1) {
  return typename std::hash<std::shared_ptr<T1>>::argument_type(a0, a1);
}

template <typename T1, typename T2,
          typename std::enable_if<std::is_convertible<T2 *, T1 *>::value,
                                  int>::type T3 = 0>
typename std::hash<std::shared_ptr<T1>>::argument_type
f232(std::auto_ptr<T2> &&a0) {
  return typename std::hash<std::shared_ptr<T1>>::argument_type(std::move(a0));
}

template <typename T1, typename T2,
          typename std::enable_if<!std::is_array<T2>::value &&
                                      std::is_convertible<T2 *, T1 *>::value,
                                  int>::type T3 = 0>
typename std::hash<std::shared_ptr<T1>>::argument_type &
f233(typename std::hash<std::shared_ptr<T1>>::argument_type &a0,
     std::auto_ptr<T2> &&a1) {
  return a0.template operator= <T2>(std::move(a1));
}

template <typename T1, typename T2>
bool f234(const typename std::hash<std::shared_ptr<T1>>::argument_type &a0,
          const std::shared_ptr<T2> &a1) {
  return a0.template owner_before<T2>(a1);
}

template <typename T1, typename T2>
bool f235(const typename std::hash<std::shared_ptr<T1>>::argument_type &a0,
          const std::weak_ptr<T2> &a1) {
  return a0.template owner_before<T2>(a1);
}

template <typename T1, typename T2>
using t48 = typename std::hash<std::__enable_hash_helper<
    std::unique_ptr<T1, T2>,
    typename std::unique_ptr<T1, T2>::pointer>>::argument_type;

template <typename T1, typename T2>
using t49 = typename std::hash<std::__enable_hash_helper<
    std::unique_ptr<T1, T2>,
    typename std::unique_ptr<T1, T2>::pointer>>::result_type;

template <typename T1, typename T2>
using t50 = typename std::hash<std::__enable_hash_helper<
    std::unique_ptr<T1, T2>,
    typename std::unique_ptr<T1, T2>::pointer>>::argument_type::element_type;

template <typename T1, typename T2>
using t51 = typename std::hash<std::__enable_hash_helper<
    std::unique_ptr<T1, T2>,
    typename std::unique_ptr<T1, T2>::pointer>>::argument_type::deleter_type;

template <typename T1, typename T2>
using t52 = typename std::hash<std::__enable_hash_helper<
    std::unique_ptr<T1, T2>,
    typename std::unique_ptr<T1, T2>::pointer>>::argument_type::pointer;

template <typename T1, typename T2>
typename std::hash<std::__enable_hash_helper<
    std::unique_ptr<T1, T2>,
    typename std::unique_ptr<T1, T2>::pointer>>::argument_type
f236(typename std::hash<std::unique_ptr<T1, T2>>::argument_type &&a0) {
  return typename std::hash<std::__enable_hash_helper<
      std::unique_ptr<T1, T2>,
      typename std::unique_ptr<T1, T2>::pointer>>::argument_type(std::move(a0));
}

template <typename T1, typename T2>
typename std::hash<std::unique_ptr<T1, T2>>::argument_type &
f237(typename std::hash<std::__enable_hash_helper<
         std::unique_ptr<T1, T2>,
         typename std::unique_ptr<T1, T2>::pointer>>::argument_type &a0,
     typename std::hash<std::unique_ptr<T1, T2>>::argument_type &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
typename std::hash<std::unique_ptr<T1, T2>>::argument_type &
f238(typename std::hash<std::__enable_hash_helper<
         std::unique_ptr<T1, T2>,
         typename std::unique_ptr<T1, T2>::pointer>>::argument_type &a0,
     std::nullptr_t a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
typename std::hash<std::unique_ptr<T1, T2>>::argument_type::pointer
f239(const typename std::hash<std::__enable_hash_helper<
         std::unique_ptr<T1, T2>,
         typename std::unique_ptr<T1, T2>::pointer>>::argument_type &a0) {
  return a0.operator->();
}

template <typename T1, typename T2>
typename std::hash<std::unique_ptr<T1, T2>>::argument_type::pointer
f240(const typename std::hash<std::__enable_hash_helper<
         std::unique_ptr<T1, T2>,
         typename std::unique_ptr<T1, T2>::pointer>>::argument_type &a0) {
  return a0.get();
}

template <typename T1, typename T2>
typename std::hash<std::unique_ptr<T1, T2>>::argument_type::deleter_type &
f241(typename std::hash<std::__enable_hash_helper<
         std::unique_ptr<T1, T2>,
         typename std::unique_ptr<T1, T2>::pointer>>::argument_type &a0) {
  return a0.get_deleter();
}

template <typename T1, typename T2>
const typename std::hash<std::unique_ptr<T1, T2>>::argument_type::deleter_type &
f242(const typename std::hash<std::__enable_hash_helper<
         std::unique_ptr<T1, T2>,
         typename std::unique_ptr<T1, T2>::pointer>>::argument_type &a0) {
  return a0.get_deleter();
}

template <typename T1, typename T2>
bool f243(const typename std::hash<std::__enable_hash_helper<
              std::unique_ptr<T1, T2>,
              typename std::unique_ptr<T1, T2>::pointer>>::argument_type &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2>
typename std::hash<std::unique_ptr<T1, T2>>::argument_type::pointer
f244(typename std::hash<std::__enable_hash_helper<
         std::unique_ptr<T1, T2>,
         typename std::unique_ptr<T1, T2>::pointer>>::argument_type &a0) {
  return a0.release();
}

template <typename T1, typename T2>
void f245(
    typename std::hash<std::__enable_hash_helper<
        std::unique_ptr<T1, T2>,
        typename std::unique_ptr<T1, T2>::pointer>>::argument_type &a0,
    typename std::hash<std::unique_ptr<T1, T2>>::argument_type::pointer a1) {
  return a0.reset(a1);
}

template <typename T1, typename T2>
void f246(typename std::hash<std::__enable_hash_helper<
              std::unique_ptr<T1, T2>,
              typename std::unique_ptr<T1, T2>::pointer>>::argument_type &a0,
          typename std::hash<std::unique_ptr<T1, T2>>::argument_type &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2,
          typename T3 = typename std::hash<
              std::unique_ptr<T1, T2>>::argument_type::deleter_type,
          typename std::enable_if<std::is_default_constructible<T2>::value &&
                                      !std::is_pointer<T2>::value,
                                  int>::type T4 = 0>
typename std::hash<std::__enable_hash_helper<
    std::unique_ptr<T1, T2>,
    typename std::unique_ptr<T1, T2>::pointer>>::argument_type
f247() {
  return typename std::hash<std::__enable_hash_helper<
      std::unique_ptr<T1, T2>,
      typename std::unique_ptr<T1, T2>::pointer>>::argument_type();
}

template <typename T1, typename T2,
          typename T3 = typename std::hash<
              std::unique_ptr<T1, T2>>::argument_type::deleter_type,
          typename std::enable_if<std::is_default_constructible<T2>::value &&
                                      !std::is_pointer<T2>::value,
                                  int>::type T4 = 0>
typename std::hash<std::__enable_hash_helper<
    std::unique_ptr<T1, T2>,
    typename std::unique_ptr<T1, T2>::pointer>>::argument_type
f248(std::nullptr_t a0) {
  return typename std::hash<std::__enable_hash_helper<
      std::unique_ptr<T1, T2>,
      typename std::unique_ptr<T1, T2>::pointer>>::argument_type(a0);
}

template <typename T1, typename T2,
          typename T3 = typename std::hash<
              std::unique_ptr<T1, T2>>::argument_type::deleter_type,
          typename std::enable_if<std::is_default_constructible<T2>::value &&
                                      !std::is_pointer<T2>::value,
                                  int>::type T4 = 0>
typename std::hash<std::__enable_hash_helper<
    std::unique_ptr<T1, T2>,
    typename std::unique_ptr<T1, T2>::pointer>>::argument_type
f249(typename std::hash<std::unique_ptr<T1, T2>>::argument_type::pointer a0) {
  return typename std::hash<std::__enable_hash_helper<
      std::unique_ptr<T1, T2>,
      typename std::unique_ptr<T1, T2>::pointer>>::argument_type(a0);
}

template <typename T1, typename T2,
          typename T3 = typename std::hash<
              std::unique_ptr<T1, T2>>::argument_type::deleter_type,
          typename std::enable_if<std::is_copy_constructible<T2>::value,
                                  int>::type T4 = 0>
typename std::hash<std::__enable_hash_helper<
    std::unique_ptr<T1, T2>,
    typename std::unique_ptr<T1, T2>::pointer>>::argument_type
f250(typename std::hash<std::unique_ptr<T1, T2>>::argument_type::pointer a0,
     const typename std::hash<
         std::unique_ptr<T1, T2>>::argument_type::deleter_type &a1) {
  return typename std::hash<std::__enable_hash_helper<
      std::unique_ptr<T1, T2>,
      typename std::unique_ptr<T1, T2>::pointer>>::argument_type(a0, a1);
}

template <typename T1, typename T2,
          typename T3 = typename std::hash<
              std::unique_ptr<T1, T2>>::argument_type::deleter_type,
          typename std::enable_if<!std::is_reference<T2>::value &&
                                      std::is_move_constructible<T2>::value,
                                  int>::type T4 = 0>
typename std::hash<std::__enable_hash_helper<
    std::unique_ptr<T1, T2>,
    typename std::unique_ptr<T1, T2>::pointer>>::argument_type
f251(typename std::hash<std::unique_ptr<T1, T2>>::argument_type::pointer a0,
     typename std::hash<std::unique_ptr<T1, T2>>::argument_type::deleter_type
         &&a1) {
  return typename std::hash<std::__enable_hash_helper<
      std::unique_ptr<T1, T2>, typename std::unique_ptr<T1, T2>::pointer>>::
      argument_type(a0, std::move(a1));
}

template <typename T1, typename T2, typename T3,
          typename std::enable_if<
              std::is_convertible<T3 *, T1 *>::value &&
                  std::is_same<T2, std::default_delete<T1>>::value,
              int>::type T4 = 0>
typename std::hash<std::__enable_hash_helper<
    std::unique_ptr<T1, T2>,
    typename std::unique_ptr<T1, T2>::pointer>>::argument_type
f252(std::auto_ptr<T3> &&a0) {
  return typename std::hash<std::__enable_hash_helper<
      std::unique_ptr<T1, T2>,
      typename std::unique_ptr<T1, T2>::pointer>>::argument_type(std::move(a0));
}

template <typename T1, typename T2, typename T3,
          typename std::enable_if<
              std::is_convertible<T3 *, T1 *>::value &&
                  std::is_same<T2, std::default_delete<T1>>::value,
              int>::type T4 = 0>
typename std::hash<std::unique_ptr<T1, T2>>::argument_type &
f253(typename std::hash<std::__enable_hash_helper<
         std::unique_ptr<T1, T2>,
         typename std::unique_ptr<T1, T2>::pointer>>::argument_type &a0,
     std::auto_ptr<T3> a1) {
  return a0.template operator= <T3>(a1);
}

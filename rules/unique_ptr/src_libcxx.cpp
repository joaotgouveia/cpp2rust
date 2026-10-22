#include "../common/init.h"
#include "../common/traits.h"
#include <cstddef>
#include <memory>
#include <utility>

template <typename T1, typename T2>
__add_lvalue_reference(T1) f66(const std::unique_ptr<T1[], T2> &a0,
                               std::size_t a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::unique_ptr<T1[], T2> f67(T3 a0, T4 a1, std::size_t a2) {
  return std::unique_ptr<T1[], T2>(a0, a1, a2);
}

template <typename T1, typename T2>
std::unique_ptr<T1[], T2>
f68(std::nullptr_t a0,
    const typename std::unique_ptr<T1[], T2>::deleter_type &a1) {
  return std::unique_ptr<T1[], T2>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::unique_ptr<T1[], T2>
f69(T3 a0, typename std::unique_ptr<T1[], T2>::deleter_type &&a1) {
  return std::unique_ptr<T1[], T2>(a0, std::move(a1));
}

template <typename T1, typename T2>
std::unique_ptr<T1[], T2>
f70(std::nullptr_t a0, typename std::unique_ptr<T1[], T2>::deleter_type &&a1) {
  return std::unique_ptr<T1[], T2>(a0, std::move(a1));
}

template <typename T1, typename T2>
std::unique_ptr<T1, T2>
f71(typename std::unique_ptr<T1, T2>::pointer a0,
    typename std::unique_ptr<T1, T2>::deleter_type &&a1) {
  return std::unique_ptr<T1, T2>(a0, std::move(a1));
}

template <typename T1, typename T2>
__add_lvalue_reference(T1) f72(const std::unique_ptr<T1, T2> &a0) {
  return a0.operator*();
}

#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <condition_variable>
#include <utility>

<<<<<<< HEAD
template <typename T1, typename T2>
bool f1(const std::unique_ptr<T1, T2> &a0, std::nullptr_t a1) {
  return a0 < a1;
=======
template <typename T1> using t1 = std::unique_ptr<T1>;
template <typename T1> using t2 = std::unique_ptr<T1[]>;
template <typename T1> using t3 = typename std::unique_ptr<T1>::pointer;
template <typename T1> using t4 = typename std::unique_ptr<T1[]>::pointer;

template <typename T1> T1 *f2(const std::unique_ptr<T1> &o) { return o.get(); }

template <typename T1> std::unique_ptr<T1> f3(T1 *p) {
  return std::unique_ptr<T1>(p);
>>>>>>> master
}

template <typename T1, typename T2>
bool f2(std::nullptr_t a0, const std::unique_ptr<T1, T2> &a1) {
  return a0 < a1;
}

template <typename T1, typename T2, typename T3, typename T4>
bool f3(const std::unique_ptr<T1, T2> &a0, const std::unique_ptr<T3, T4> &a1) {
  return a0 < a1;
}

template <typename T1, typename T2>
bool f4(const std::unique_ptr<T1, T2> &a0, std::nullptr_t a1) {
  return a0 <= a1;
}

template <typename T1, typename T2>
bool f5(std::nullptr_t a0, const std::unique_ptr<T1, T2> &a1) {
  return a0 <= a1;
}

template <typename T1, typename T2, typename T3, typename T4>
bool f6(const std::unique_ptr<T1, T2> &a0, const std::unique_ptr<T3, T4> &a1) {
  return a0 <= a1;
}

template <typename T1, typename T2>
bool f7(const std::unique_ptr<T1, T2> &a0, std::nullptr_t a1) {
  return a0 == a1;
}

template <typename T1, typename T2, typename T3, typename T4>
bool f8(const std::unique_ptr<T1, T2> &a0, const std::unique_ptr<T3, T4> &a1) {
  return a0 == a1;
}

template <typename T1, typename T2>
bool f9(const std::unique_ptr<T1, T2> &a0, std::nullptr_t a1) {
  return a0 > a1;
}

template <typename T1, typename T2>
bool f10(std::nullptr_t a0, const std::unique_ptr<T1, T2> &a1) {
  return a0 > a1;
}

template <typename T1, typename T2, typename T3, typename T4>
bool f11(const std::unique_ptr<T1, T2> &a0, const std::unique_ptr<T3, T4> &a1) {
  return a0 > a1;
}

template <typename T1, typename T2>
bool f12(const std::unique_ptr<T1, T2> &a0, std::nullptr_t a1) {
  return a0 >= a1;
}

template <typename T1, typename T2>
bool f13(std::nullptr_t a0, const std::unique_ptr<T1, T2> &a1) {
  return a0 >= a1;
}

template <typename T1, typename T2, typename T3, typename T4>
bool f14(const std::unique_ptr<T1, T2> &a0, const std::unique_ptr<T3, T4> &a1) {
  return a0 >= a1;
}

template <typename T1, typename T2>
void f15(std::unique_ptr<T1, T2> &a0, std::unique_ptr<T1, T2> &a1) {
  return std::swap<T1, T2>(a0, a1);
}

template <typename T1, typename T2> using t1 = std::unique_ptr<T1[], T2>;

template <typename T1, typename T2>
using t2 = typename std::unique_ptr<T1[], T2>::pointer;

template <typename T1, typename T2>
using t3 = typename std::unique_ptr<T1[], T2>::element_type;

template <typename T1, typename T2>
using t4 = typename std::unique_ptr<T1[], T2>::deleter_type;

template <typename T1, typename T2>
std::unique_ptr<T1[], T2> f16(std::unique_ptr<T1[], T2> &&a0) {
  return std::unique_ptr<T1[], T2>(std::move(a0));
}

template <typename T1, typename T2>
std::unique_ptr<T1[], T2> &f17(std::unique_ptr<T1[], T2> &a0,
                               std::unique_ptr<T1[], T2> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
std::unique_ptr<T1[], T2> &f18(std::unique_ptr<T1[], T2> &a0,
                               std::nullptr_t a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
typename std::add_lvalue_reference<
    typename std::unique_ptr<T1[], T2>::element_type>::type
f19(const std::unique_ptr<T1[], T2> &a0, std::size_t a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2>
typename std::unique_ptr<T1[], T2>::pointer
f20(const std::unique_ptr<T1[], T2> &a0) {
  return a0.get();
}

template <typename T1, typename T2>
typename std::unique_ptr<T1[], T2>::deleter_type &
f21(std::unique_ptr<T1[], T2> &a0) {
  return a0.get_deleter();
}

template <typename T1, typename T2>
const typename std::unique_ptr<T1[], T2>::deleter_type &
f22(const std::unique_ptr<T1[], T2> &a0) {
  return a0.get_deleter();
}

template <typename T1, typename T2>
bool f23(const std::unique_ptr<T1[], T2> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2>
typename std::unique_ptr<T1[], T2>::pointer f24(std::unique_ptr<T1[], T2> &a0) {
  return a0.release();
}

template <typename T1, typename T2>
void f25(std::unique_ptr<T1[], T2> &a0, std::nullptr_t a1) {
  return a0.reset(a1);
}

template <typename T1, typename T2>
void f26(std::unique_ptr<T1[], T2> &a0, std::unique_ptr<T1[], T2> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2> std::unique_ptr<T1[], T2> f27() {
  return std::unique_ptr<T1[], T2>();
}

template <typename T1, typename T2, typename T3>
std::unique_ptr<T1[], T2> f28(T3 a0) {
  return std::unique_ptr<T1[], T2>(a0);
}

template <typename T1, typename T2, typename T3>
std::unique_ptr<T1[], T2>
f29(T3 a0, const typename std::unique_ptr<T1[], T2>::deleter_type &a1) {
  return std::unique_ptr<T1[], T2>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::unique_ptr<T1[], T2> f30(T3 a0, T4 &&a1) {
  return std::unique_ptr<T1[], T2>(a0, std::move(a1));
}

template <typename T1, typename T2>
std::unique_ptr<T1[], T2> f31(std::nullptr_t a0) {
  return std::unique_ptr<T1[], T2>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::unique_ptr<T1[], T2> f32(std::unique_ptr<T3, T4> &&a0) {
  return std::unique_ptr<T1[], T2>(std::move(a0));
}

template <typename T1, typename T2, typename T3, typename T4>
std::unique_ptr<T1[], T2> &f33(std::unique_ptr<T1[], T2> &a0,
                               std::unique_ptr<T3, T4> &&a1) {
  return a0.template operator= <T3, T4>(std::move(a1));
}

template <typename T1, typename T2, typename T3>
void f34(std::unique_ptr<T1[], T2> &a0, T3 a1) {
  return a0.template reset<T3>(a1);
}

template <typename T1, typename T2> using t5 = std::unique_ptr<T1, T2>;

template <typename T1, typename T2>
using t6 = typename std::unique_ptr<T1, T2>::pointer;

template <typename T1, typename T2>
using t7 = typename std::unique_ptr<T1, T2>::element_type;

template <typename T1, typename T2>
using t8 = typename std::unique_ptr<T1, T2>::deleter_type;

template <typename T1, typename T2>
std::unique_ptr<T1, T2> f35(std::unique_ptr<T1, T2> &&a0) {
  return std::unique_ptr<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::unique_ptr<T1, T2> &f36(std::unique_ptr<T1, T2> &a0,
                             std::unique_ptr<T1, T2> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
std::unique_ptr<T1, T2> &f37(std::unique_ptr<T1, T2> &a0, std::nullptr_t a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
typename std::add_lvalue_reference<
    typename std::unique_ptr<T1, T2>::element_type>::type
f38(const std::unique_ptr<T1, T2> &a0) {
  return a0.operator*();
}

template <typename T1, typename T2>
typename std::unique_ptr<T1, T2>::pointer
f39(const std::unique_ptr<T1, T2> &a0) {
  return a0.operator->();
}

template <typename T1, typename T2>
typename std::unique_ptr<T1, T2>::pointer
f40(const std::unique_ptr<T1, T2> &a0) {
  return a0.get();
}

template <typename T1, typename T2>
typename std::unique_ptr<T1, T2>::deleter_type &
f41(std::unique_ptr<T1, T2> &a0) {
  return a0.get_deleter();
}

template <typename T1, typename T2>
const typename std::unique_ptr<T1, T2>::deleter_type &
f42(const std::unique_ptr<T1, T2> &a0) {
  return a0.get_deleter();
}

template <typename T1, typename T2>
bool f43(const std::unique_ptr<T1, T2> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2>
typename std::unique_ptr<T1, T2>::pointer f44(std::unique_ptr<T1, T2> &a0) {
  return a0.release();
}

template <typename T1, typename T2>
void f45(std::unique_ptr<T1, T2> &a0,
         typename std::unique_ptr<T1, T2>::pointer a1) {
  return a0.reset(a1);
}

template <typename T1, typename T2>
void f46(std::unique_ptr<T1, T2> &a0, std::unique_ptr<T1, T2> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2> std::unique_ptr<T1, T2> f47() {
  return std::unique_ptr<T1, T2>();
}

template <typename T1, typename T2>
std::unique_ptr<T1, T2> f48(typename std::unique_ptr<T1, T2>::pointer a0) {
  return std::unique_ptr<T1, T2>(a0);
}

template <typename T1, typename T2>
std::unique_ptr<T1, T2>
f49(typename std::unique_ptr<T1, T2>::pointer a0,
    const typename std::unique_ptr<T1, T2>::deleter_type &a1) {
  return std::unique_ptr<T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::unique_ptr<T1, T2> f50(typename std::unique_ptr<T1, T2>::pointer a0,
                            T3 &&a1) {
  return std::unique_ptr<T1, T2>(a0, std::move(a1));
}

template <typename T1, typename T2>
std::unique_ptr<T1, T2> f51(std::nullptr_t a0) {
  return std::unique_ptr<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::unique_ptr<T1, T2> f52(std::unique_ptr<T3, T4> &&a0) {
  return std::unique_ptr<T1, T2>(std::move(a0));
}

template <typename T1, typename T2, typename T3>
std::unique_ptr<T1, T2> f53(std::auto_ptr<T3> &&a0) {
  return std::unique_ptr<T1, T2>(std::move(a0));
}

template <typename T1, typename T2, typename T3, typename T4>
std::unique_ptr<T1, T2> &f54(std::unique_ptr<T1, T2> &a0,
                             std::unique_ptr<T3, T4> &&a1) {
  return a0.template operator= <T3, T4>(std::move(a1));
}

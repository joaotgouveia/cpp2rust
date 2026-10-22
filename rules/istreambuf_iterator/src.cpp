#include "../common/init.h"
#include "../common/traits.h"
#include <ios>
#include <iosfwd>
#include <utility>

template <typename T1, typename T2> using t1 = std::istreambuf_iterator<T1, T2>;

template <typename T1, typename T2>
using t2 = typename std::istreambuf_iterator<T1, T2>::char_type;

template <typename T1, typename T2>
using t3 = typename std::istreambuf_iterator<T1, T2>::traits_type;

template <typename T1, typename T2>
using t4 = typename std::istreambuf_iterator<T1, T2>::int_type;

template <typename T1, typename T2>
using t5 = typename std::istreambuf_iterator<T1, T2>::streambuf_type;

template <typename T1, typename T2>
using t6 = typename std::istreambuf_iterator<T1, T2>::istream_type;

template <typename T1, typename T2> std::istreambuf_iterator<T1, T2> f1() {
  return std::istreambuf_iterator<T1, T2>();
}

template <typename T1, typename T2>
std::istreambuf_iterator<T1, T2>
f2(const std::istreambuf_iterator<T1, T2> &a0) {
  return std::istreambuf_iterator<T1, T2>(a0);
}

template <typename T1, typename T2>
std::istreambuf_iterator<T1, T2>
f3(typename std::istreambuf_iterator<T1, T2>::istream_type &a0) {
  return std::istreambuf_iterator<T1, T2>(a0);
}

template <typename T1, typename T2>
std::istreambuf_iterator<T1, T2>
f4(typename std::istreambuf_iterator<T1, T2>::streambuf_type *a0) {
  return std::istreambuf_iterator<T1, T2>(a0);
}

template <typename T1, typename T2>
std::istreambuf_iterator<T1, T2> &
f5(std::istreambuf_iterator<T1, T2> &a0,
   const std::istreambuf_iterator<T1, T2> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
typename std::istreambuf_iterator<T1, T2>::char_type
f6(const std::istreambuf_iterator<T1, T2> &a0) {
  return a0.operator*();
}

template <typename T1, typename T2>
std::istreambuf_iterator<T1, T2> &f7(std::istreambuf_iterator<T1, T2> &a0) {
  return a0.operator++();
}

template <typename T1, typename T2>
std::istreambuf_iterator<T1, T2> f8(std::istreambuf_iterator<T1, T2> &a0,
                                    int a1) {
  return a0.operator++(a1);
}

template <typename T1, typename T2>
bool f9(const std::istreambuf_iterator<T1, T2> &a0,
        const std::istreambuf_iterator<T1, T2> &a1) {
  return a0.equal(a1);
}

template <typename T1, typename T2>
using t7 = typename std::istreambuf_iterator<T1, T2>::streambuf_type::char_type;

template <typename T1, typename T2>
using t8 =
    typename std::istreambuf_iterator<T1, T2>::streambuf_type::traits_type;

template <typename T1, typename T2>
using t9 = typename std::istreambuf_iterator<T1, T2>::streambuf_type::int_type;

template <typename T1, typename T2>
using t10 = typename std::istreambuf_iterator<T1, T2>::streambuf_type::pos_type;

template <typename T1, typename T2>
using t11 = typename std::istreambuf_iterator<T1, T2>::streambuf_type::off_type;

template <typename T1, typename T2>
std::basic_ostream<T1, T2> *
f10(const typename std::istreambuf_iterator<T1, T2>::istream_type &a0) {
  return a0.tie();
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> *
f11(typename std::istreambuf_iterator<T1, T2>::istream_type &a0,
    std::basic_ostream<T1, T2> *a1) {
  return a0.tie(a1);
}

template <typename T1, typename T2>
std::basic_streambuf<T1, T2> *
f12(const typename std::istreambuf_iterator<T1, T2>::istream_type &a0) {
  return a0.rdbuf();
}

template <typename T1, typename T2>
std::basic_streambuf<T1, T2> *
f13(typename std::istreambuf_iterator<T1, T2>::istream_type &a0,
    std::basic_streambuf<T1, T2> *a1) {
  return a0.rdbuf(a1);
}

template <typename T1, typename T2> bool f14(bool a0) {
  return std::istreambuf_iterator<T1, T2>::istream_type::sync_with_stdio(a0);
}

template <typename T1, typename T2> int f15() {
  return std::istreambuf_iterator<T1, T2>::istream_type::xalloc();
}

#include <algorithm>
#include <ios>
#include <iosfwd>
#include <utility>

template <typename T1, typename T2> using t1 = std::istreambuf_iterator<T1, T2>;

template <typename T1, typename T2>
using t2 = typename std::istreambuf_iterator<T1, T2>::pointer;

template <typename T1, typename T2>
using t3 = typename std::istreambuf_iterator<T1, T2>::char_type;

template <typename T1, typename T2>
using t4 = typename std::istreambuf_iterator<T1, T2>::traits_type;

template <typename T1, typename T2>
using t5 = typename std::istreambuf_iterator<T1, T2>::int_type;

template <typename T1, typename T2>
using t6 = typename std::istreambuf_iterator<T1, T2>::streambuf_type;

template <typename T1, typename T2>
using t7 = typename std::istreambuf_iterator<T1, T2>::istream_type;

template <typename T1, typename T2> std::istreambuf_iterator<T1, T2> f1() {
  return std::istreambuf_iterator<T1, T2>();
}

template <typename T1, typename T2>
std::istreambuf_iterator<T1, T2> f2(std::default_sentinel_t a0) {
  return std::istreambuf_iterator<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::istreambuf_iterator<T1, T2>
f3(const std::istreambuf_iterator<T1, T2> &a0) {
  return std::istreambuf_iterator<T1, T2>(a0);
}

template <typename T1, typename T2>
std::istreambuf_iterator<T1, T2> f4(std::basic_istream<T1, T2> &a0) {
  return std::istreambuf_iterator<T1, T2>(a0);
}

template <typename T1, typename T2>
std::istreambuf_iterator<T1, T2> f5(std::basic_streambuf<T1, T2> *a0) {
  return std::istreambuf_iterator<T1, T2>(a0);
}

template <typename T1, typename T2>
std::istreambuf_iterator<T1, T2> &
f6(std::istreambuf_iterator<T1, T2> &a0,
   const std::istreambuf_iterator<T1, T2> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
T1 f7(const std::istreambuf_iterator<T1, T2> &a0) {
  return a0.operator*();
}

template <typename T1, typename T2>
std::istreambuf_iterator<T1, T2> &f8(std::istreambuf_iterator<T1, T2> &a0) {
  return a0.operator++();
}

template <typename T1, typename T2>
std::istreambuf_iterator<T1, T2> f9(std::istreambuf_iterator<T1, T2> &a0,
                                    int a1) {
  return a0.operator++(a1);
}

template <typename T1, typename T2>
bool f10(const std::istreambuf_iterator<T1, T2> &a0,
         const std::istreambuf_iterator<T1, T2> &a1) {
  return a0.equal(a1);
}

template <typename T1, typename T2>
bool f11(const std::istreambuf_iterator<T1, T2> &a0,
         std::default_sentinel_t a1) {
  return operator==(a0, std::move(a1));
}

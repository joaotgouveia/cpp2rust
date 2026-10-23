#include <ios>
#include <iosfwd>
#include <iterator>
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
f2(typename std::istreambuf_iterator<T1, T2>::istream_type &a0) {
  return std::istreambuf_iterator<T1, T2>(a0);
}

template <typename T1, typename T2>
std::istreambuf_iterator<T1, T2>
f3(typename std::istreambuf_iterator<T1, T2>::streambuf_type *a0) {
  return std::istreambuf_iterator<T1, T2>(a0);
}

template <typename T1, typename T2>
typename std::istreambuf_iterator<T1, T2>::char_type
f4(const std::istreambuf_iterator<T1, T2> &a0) {
  return a0.operator*();
}

template <typename T1, typename T2>
std::istreambuf_iterator<T1, T2> &f5(std::istreambuf_iterator<T1, T2> &a0) {
  return a0.operator++();
}

template <typename T1, typename T2>
bool f6(const std::istreambuf_iterator<T1, T2> &a0,
        const std::istreambuf_iterator<T1, T2> &a1) {
  return a0.equal(a1);
}

template <typename T1, typename T2> bool f7(bool a0) {
  return std::istreambuf_iterator<T1, T2>::istream_type::sync_with_stdio(a0);
}

template <typename T1, typename T2> int f8() {
  return std::istreambuf_iterator<T1, T2>::istream_type::xalloc();
}

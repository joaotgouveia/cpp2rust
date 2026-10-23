#include <ios>
#include <iosfwd>
#include <utility>

template <typename T1, typename T2>
using t6 = typename std::ostreambuf_iterator<T1, T2>::streambuf_type::char_type;

template <typename T1, typename T2>
using t7 =
    typename std::ostreambuf_iterator<T1, T2>::streambuf_type::traits_type;

template <typename T1, typename T2>
using t8 = typename std::ostreambuf_iterator<T1, T2>::streambuf_type::int_type;

template <typename T1, typename T2>
using t9 = typename std::ostreambuf_iterator<T1, T2>::streambuf_type::pos_type;

template <typename T1, typename T2>
using t10 = typename std::ostreambuf_iterator<T1, T2>::streambuf_type::off_type;

template <typename T1, typename T2>
std::basic_ostream<T1, T2> *
f10(const typename std::ostreambuf_iterator<T1, T2>::ostream_type &a0) {
  return a0.tie();
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> *
f11(typename std::ostreambuf_iterator<T1, T2>::ostream_type &a0,
    std::basic_ostream<T1, T2> *a1) {
  return a0.tie(a1);
}

template <typename T1, typename T2>
std::basic_streambuf<T1, T2> *
f12(const typename std::ostreambuf_iterator<T1, T2>::ostream_type &a0) {
  return a0.rdbuf();
}

template <typename T1, typename T2>
std::basic_streambuf<T1, T2> *
f13(typename std::ostreambuf_iterator<T1, T2>::ostream_type &a0,
    std::basic_streambuf<T1, T2> *a1) {
  return a0.rdbuf(a1);
}

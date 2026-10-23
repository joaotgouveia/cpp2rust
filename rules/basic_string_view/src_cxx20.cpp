#include <string_view>
#include <utility>

template <typename T1, typename T2>
bool f1(const std::basic_string_view<T1, T2> &a0,
        std::basic_string_view<T1, T2> a1) {
  return a0.starts_with(a1);
}

template <typename T1, typename T2>
bool f2(const std::basic_string_view<T1, T2> &a0,
        std::basic_string_view<T1, T2> a1) {
  return a0.ends_with(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string_view<T1, T2> f3(T3 a0, T4 a1) {
  return std::basic_string_view<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
using t1 = typename std::basic_string_view<
    T1, T2>::const_reverse_iterator::iterator_concept;

template <typename T1, typename T2>
using t2 =
    typename std::basic_string_view<T1, T2>::reverse_iterator::iterator_concept;

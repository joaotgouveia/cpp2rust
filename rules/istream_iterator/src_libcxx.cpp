#include <istream>
#include <iterator>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
bool f14(const std::istream_iterator<T1, T2, T3, T4> &a0,
         const std::istream_iterator<T1, T2, T3, T4> &a1) {
  return std::operator== <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
using t5 = typename std::istream_iterator<T1, T2, T3, T4>::iterator_category;

template <typename T1, typename T2, typename T3, typename T4>
using t6 = typename std::istream_iterator<T1, T2, T3, T4>::value_type;

template <typename T1, typename T2, typename T3, typename T4>
using t7 = typename std::istream_iterator<T1, T2, T3, T4>::difference_type;

template <typename T1, typename T2, typename T3, typename T4>
using t8 = typename std::istream_iterator<T1, T2, T3, T4>::pointer;

template <typename T1, typename T2, typename T3, typename T4>
using t9 = typename std::istream_iterator<T1, T2, T3, T4>::reference;

#include <initializer_list>
#include <string>
#include <string_view>
#include <utility>

template <typename T1, typename T2, typename T3>
using t25 = typename std::basic_string<T1, T2, T3>::iterator;

template <typename T1, typename T2, typename T3>
using t26 = typename std::basic_string<T1, T2, T3>::const_iterator;

template <typename T1, typename T2, typename T3>
using t27 = typename std::basic_string<T1, T2, T3>::reverse_iterator;

template <typename T1, typename T2, typename T3>
using t28 = typename std::basic_string<T1, T2, T3>::const_reverse_iterator;

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f279() {
  return std::basic_string<T1, T2, T3>();
}

template <typename T1, typename T2, typename T3>
std::basic_string_view<T1, T2> f280(const std::basic_string<T1, T2, T3> &a0) {
  return static_cast<std::basic_string_view<T1, T2>>(a0);
}

template <typename T1, typename T2, typename T3>
const T1 &f281(const std::basic_string<T1, T2, T3> &a0,
               typename std::basic_string<T1, T2, T3>::size_type a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2, typename T3>
T1 &f282(std::basic_string<T1, T2, T3> &a0,
         typename std::basic_string<T1, T2, T3>::size_type a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2, typename T3>
const T1 &f283(const std::basic_string<T1, T2, T3> &a0,
               typename std::basic_string<T1, T2, T3>::size_type a1) {
  return a0.at(a1);
}

template <typename T1, typename T2, typename T3>
T1 &f284(std::basic_string<T1, T2, T3> &a0,
         typename std::basic_string<T1, T2, T3>::size_type a1) {
  return a0.at(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f285(std::basic_string<T1, T2, T3> &a0, const T1 *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.append(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
T1 &f286(std::basic_string<T1, T2, T3> &a0) {
  return a0.front();
}

template <typename T1, typename T2, typename T3>
const T1 &f287(const std::basic_string<T1, T2, T3> &a0) {
  return a0.front();
}

template <typename T1, typename T2, typename T3>
T1 &f288(std::basic_string<T1, T2, T3> &a0) {
  return a0.back();
}

template <typename T1, typename T2, typename T3>
const T1 &f289(const std::basic_string<T1, T2, T3> &a0) {
  return a0.back();
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f290(std::basic_string<T1, T2, T3> &a0, const T1 *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.assign(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
T3 f291(const std::basic_string<T1, T2, T3> &a0) {
  return a0.get_allocator();
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<std::__is_allocator_v<T3>, int> T4 = 0>
std::basic_string<T1, T2, T3> f292(const T1 *a0, const T3 &a1) {
  return std::basic_string<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<std::__is_allocator_v<T3>, int> T4 = 0>
std::basic_string<T1, T2, T3>
f293(typename std::basic_string<T1, T2, T3>::size_type a0, T1 a1,
     const T3 &a2) {
  return std::basic_string<T1, T2, T3>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<std::__has_input_iterator_category<T4>::value, int>
              T5 = 0>
std::basic_string<T1, T2, T3> f294(T4 a0, T4 a1, const T3 &a2) {
  return std::basic_string<T1, T2, T3>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<
              std::__has_exactly_input_iterator_category<T4>::value, int>
              T5 = 0>
std::basic_string<T1, T2, T3> &f295(std::basic_string<T1, T2, T3> &a0, T4 a1,
                                    T4 a2) {
  return a0.template append<T4>(a1, a2);
}

template <
    typename T1, typename T2, typename T3, typename T4,
    std::__enable_if_t<std::__has_forward_iterator_category<T4>::value, int>
        T5 = 0>
std::basic_string<T1, T2, T3> &f296(std::basic_string<T1, T2, T3> &a0, T4 a1,
                                    T4 a2) {
  return a0.template append<T4>(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<
              std::__has_exactly_input_iterator_category<T4>::value, int>
              T5 = 0>
std::basic_string<T1, T2, T3> &f297(std::basic_string<T1, T2, T3> &a0, T4 a1,
                                    T4 a2) {
  return a0.template assign<T4>(a1, a2);
}

template <
    typename T1, typename T2, typename T3, typename T4,
    std::__enable_if_t<std::__has_forward_iterator_category<T4>::value, int>
        T5 = 0>
std::basic_string<T1, T2, T3> &f298(std::basic_string<T1, T2, T3> &a0, T4 a1,
                                    T4 a2) {
  return a0.template assign<T4>(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<
              std::__has_exactly_input_iterator_category<T4>::value, int>
              T5 = 0>
typename std::basic_string<T1, T2, T3>::iterator
f299(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::const_iterator a1, T4 a2, T4 a3) {
  return a0.template insert<T4>(a1, a2, a3);
}

template <
    typename T1, typename T2, typename T3, typename T4,
    std::__enable_if_t<std::__has_forward_iterator_category<T4>::value, int>
        T5 = 0>
typename std::basic_string<T1, T2, T3>::iterator
f300(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::const_iterator a1, T4 a2, T4 a3) {
  return a0.template insert<T4>(a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<std::__has_input_iterator_category<T4>::value, int>
              T5 = 0>
std::basic_string<T1, T2, T3> &
f301(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::const_iterator a1,
     typename std::basic_string<T1, T2, T3>::const_iterator a2, T4 a3, T4 a4) {
  return a0.template replace<T4>(a1, a2, a3, a4);
}

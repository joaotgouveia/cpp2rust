#include <initializer_list>
#include <string>
#include <string_view>
#include <utility>

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f234() {
  return std::basic_string<T1, T2, T3>();
}

template <typename T1, typename T2, typename T3>
std::basic_string_view<T1, T2> f235(const std::basic_string<T1, T2, T3> &a0) {
  return static_cast<std::basic_string_view<T1, T2>>(a0);
}

template <typename T1, typename T2, typename T3>
const T1 &f236(const std::basic_string<T1, T2, T3> &a0,
               typename std::basic_string<T1, T2, T3>::size_type a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2, typename T3>
T1 &f237(std::basic_string<T1, T2, T3> &a0,
         typename std::basic_string<T1, T2, T3>::size_type a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2, typename T3>
const T1 &f238(const std::basic_string<T1, T2, T3> &a0,
               typename std::basic_string<T1, T2, T3>::size_type a1) {
  return a0.at(a1);
}

template <typename T1, typename T2, typename T3>
T1 &f239(std::basic_string<T1, T2, T3> &a0,
         typename std::basic_string<T1, T2, T3>::size_type a1) {
  return a0.at(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f240(std::basic_string<T1, T2, T3> &a0, const T1 *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.append(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
T1 &f241(std::basic_string<T1, T2, T3> &a0) {
  return a0.front();
}

template <typename T1, typename T2, typename T3>
const T1 &f242(const std::basic_string<T1, T2, T3> &a0) {
  return a0.front();
}

template <typename T1, typename T2, typename T3>
T1 &f243(std::basic_string<T1, T2, T3> &a0) {
  return a0.back();
}

template <typename T1, typename T2, typename T3>
const T1 &f244(const std::basic_string<T1, T2, T3> &a0) {
  return a0.back();
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f245(std::basic_string<T1, T2, T3> &a0, const T1 *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.assign(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
T3 f246(const std::basic_string<T1, T2, T3> &a0) {
  return a0.get_allocator();
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<std::__is_allocator_v<T3>, int> T4 = 0>
std::basic_string<T1, T2, T3> f247(const T1 *a0, const T3 &a1) {
  return std::basic_string<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<std::__has_input_iterator_category<T4>::value, int>
              T5 = 0>
std::basic_string<T1, T2, T3> f248(T4 a0, T4 a1, const T3 &a2) {
  return std::basic_string<T1, T2, T3>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<
              std::__has_exactly_input_iterator_category<T4>::value, int>
              T5 = 0>
std::basic_string<T1, T2, T3> &f249(std::basic_string<T1, T2, T3> &a0, T4 a1,
                                    T4 a2) {
  return a0.template append<T4>(a1, a2);
}

template <
    typename T1, typename T2, typename T3, typename T4,
    std::__enable_if_t<std::__has_forward_iterator_category<T4>::value, int>
        T5 = 0>
std::basic_string<T1, T2, T3> &f250(std::basic_string<T1, T2, T3> &a0, T4 a1,
                                    T4 a2) {
  return a0.template append<T4>(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<
              std::__has_exactly_input_iterator_category<T4>::value, int>
              T5 = 0>
std::basic_string<T1, T2, T3> &f251(std::basic_string<T1, T2, T3> &a0, T4 a1,
                                    T4 a2) {
  return a0.template assign<T4>(a1, a2);
}

template <
    typename T1, typename T2, typename T3, typename T4,
    std::__enable_if_t<std::__has_forward_iterator_category<T4>::value, int>
        T5 = 0>
std::basic_string<T1, T2, T3> &f252(std::basic_string<T1, T2, T3> &a0, T4 a1,
                                    T4 a2) {
  return a0.template assign<T4>(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<
              std::__has_exactly_input_iterator_category<T4>::value, int>
              T5 = 0>
typename std::basic_string<T1, T2, T3>::iterator
f253(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::const_iterator a1, T4 a2, T4 a3) {
  return a0.template insert<T4>(a1, a2, a3);
}

template <
    typename T1, typename T2, typename T3, typename T4,
    std::__enable_if_t<std::__has_forward_iterator_category<T4>::value, int>
        T5 = 0>
typename std::basic_string<T1, T2, T3>::iterator
f254(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::const_iterator a1, T4 a2, T4 a3) {
  return a0.template insert<T4>(a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<std::__has_input_iterator_category<T4>::value, int>
              T5 = 0>
std::basic_string<T1, T2, T3> &
f255(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::const_iterator a1,
     typename std::basic_string<T1, T2, T3>::const_iterator a2, T4 a3, T4 a4) {
  return a0.template replace<T4>(a1, a2, a3, a4);
}

#include <initializer_list>
#include <string>
#include <string_view>
#include <utility>

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f275() {
  return std::basic_string<T1, T2, T3>();
}

template <typename T1, typename T2, typename T3>
std::basic_string_view<T1, T2> f276(const std::basic_string<T1, T2, T3> &a0) {
  return static_cast<std::basic_string_view<T1, T2>>(a0);
}

template <typename T1, typename T2, typename T3>
const T1 &f277(const std::basic_string<T1, T2, T3> &a0,
               typename std::basic_string<T1, T2, T3>::size_type a1) {
  return a0.operator[](std::move(a1));
}

template <typename T1, typename T2, typename T3>
T1 &f278(std::basic_string<T1, T2, T3> &a0,
         typename std::basic_string<T1, T2, T3>::size_type a1) {
  return a0.operator[](std::move(a1));
}

template <typename T1, typename T2, typename T3>
const T1 &f279(const std::basic_string<T1, T2, T3> &a0,
               typename std::basic_string<T1, T2, T3>::size_type a1) {
  return a0.at(std::move(a1));
}

template <typename T1, typename T2, typename T3>
T1 &f280(std::basic_string<T1, T2, T3> &a0,
         typename std::basic_string<T1, T2, T3>::size_type a1) {
  return a0.at(std::move(a1));
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f281(std::basic_string<T1, T2, T3> &a0, const T1 *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.append(a1, std::move(a2), std::move(a3));
}

template <typename T1, typename T2, typename T3>
T1 &f282(std::basic_string<T1, T2, T3> &a0) {
  return a0.front();
}

template <typename T1, typename T2, typename T3>
const T1 &f283(const std::basic_string<T1, T2, T3> &a0) {
  return a0.front();
}

template <typename T1, typename T2, typename T3>
T1 &f284(std::basic_string<T1, T2, T3> &a0) {
  return a0.back();
}

template <typename T1, typename T2, typename T3>
const T1 &f285(const std::basic_string<T1, T2, T3> &a0) {
  return a0.back();
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f286(std::basic_string<T1, T2, T3> &a0, const T1 *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.assign(a1, std::move(a2), std::move(a3));
}

template <typename T1, typename T2, typename T3>
T3 f287(const std::basic_string<T1, T2, T3> &a0) {
  return a0.get_allocator();
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<std::__is_allocator_v<T3>, int> T4 = 0>
std::basic_string<T1, T2, T3> f288(const T1 *a0, const T3 &a1) {
  return std::basic_string<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<std::__is_allocator_v<T3>, int> T4 = 0>
std::basic_string<T1, T2, T3>
f289(typename std::basic_string<T1, T2, T3>::size_type a0, T1 a1,
     const T3 &a2) {
  return std::basic_string<T1, T2, T3>(std::move(a0), std::move(a1), a2);
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<std::__has_input_iterator_category<T4>::value, int>
              T5 = 0>
std::basic_string<T1, T2, T3> f290(T4 a0, T4 a1, const T3 &a2) {
  return std::basic_string<T1, T2, T3>(std::move(a0), std::move(a1), a2);
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<
              std::__has_exactly_input_iterator_category<T4>::value, int>
              T5 = 0>
std::basic_string<T1, T2, T3> &f291(std::basic_string<T1, T2, T3> &a0, T4 a1,
                                    T4 a2) {
  return a0.template append<T4>(std::move(a1), std::move(a2));
}

template <
    typename T1, typename T2, typename T3, typename T4,
    std::__enable_if_t<std::__has_forward_iterator_category<T4>::value, int>
        T5 = 0>
std::basic_string<T1, T2, T3> &f292(std::basic_string<T1, T2, T3> &a0, T4 a1,
                                    T4 a2) {
  return a0.template append<T4>(std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<
              std::__has_exactly_input_iterator_category<T4>::value, int>
              T5 = 0>
std::basic_string<T1, T2, T3> &f293(std::basic_string<T1, T2, T3> &a0, T4 a1,
                                    T4 a2) {
  return a0.template assign<T4>(std::move(a1), std::move(a2));
}

template <
    typename T1, typename T2, typename T3, typename T4,
    std::__enable_if_t<std::__has_forward_iterator_category<T4>::value, int>
        T5 = 0>
std::basic_string<T1, T2, T3> &f294(std::basic_string<T1, T2, T3> &a0, T4 a1,
                                    T4 a2) {
  return a0.template assign<T4>(std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<
              std::__has_exactly_input_iterator_category<T4>::value, int>
              T5 = 0>
typename std::basic_string<T1, T2, T3>::iterator
f295(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::const_iterator a1, T4 a2, T4 a3) {
  return a0.template insert<T4>(std::move(a1), std::move(a2), std::move(a3));
}

template <
    typename T1, typename T2, typename T3, typename T4,
    std::__enable_if_t<std::__has_forward_iterator_category<T4>::value, int>
        T5 = 0>
typename std::basic_string<T1, T2, T3>::iterator
f296(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::const_iterator a1, T4 a2, T4 a3) {
  return a0.template insert<T4>(std::move(a1), std::move(a2), std::move(a3));
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<std::__has_input_iterator_category<T4>::value, int>
              T5 = 0>
std::basic_string<T1, T2, T3> &
f297(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::const_iterator a1,
     typename std::basic_string<T1, T2, T3>::const_iterator a2, T4 a3, T4 a4) {
  return a0.template replace<T4>(std::move(a1), std::move(a2), std::move(a3),
                                 std::move(a4));
}

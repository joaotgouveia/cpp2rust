#include <initializer_list>
#include <ios>
#include <locale>
#include <string>
#include <utility>

template <typename T1>
std::basic_string_view<T1, std::char_traits<T1>>
f74(const typename std::messages<T1>::string_type &a0) {
  return static_cast<std::basic_string_view<T1, std::char_traits<T1>>>(a0);
}

template <typename T1> T1 *f75(typename std::messages<T1>::string_type &a0) {
  return a0.data();
}

template <typename T1, typename T2,
          typename T3 = std::enable_if_t<std::is_convertible_v<
              const T2 &, std::basic_string_view<T1, std::char_traits<T1>>>>>
typename std::messages<T1>::string_type
f76(const T2 &a0, typename std::messages<T1>::string_type::size_type a1,
    typename std::messages<T1>::string_type::size_type a2,
    const std::allocator<T1> &a3) {
  return typename std::messages<T1>::string_type(a0, a1, a2, a3);
}

template <
    typename T1, typename T2,
    typename T3 = std::enable_if_t<
        std::integral_constant<
            bool, std::is_convertible<T2 const &,
                                      std::basic_string_view<
                                          T1, std::char_traits<T1>>>::value &&
                      std::integral_constant<
                          bool, !std::is_convertible<
                                    T2 const *,
                                    const std::basic_string<
                                        T1, std::char_traits<T1>,
                                        std::allocator<T1>> *>::value>::value &&
                      std::integral_constant<
                          bool, !std::is_convertible<T2 const &, T1 const *>::
                                    value>::value>::value,
        void>>
typename std::messages<T1>::string_type f77(const T2 &a0,
                                            const std::allocator<T1> &a1) {
  return typename std::messages<T1>::string_type(a0, a1);
}

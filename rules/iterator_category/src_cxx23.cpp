#include <iterator>
#include <type_traits>
#include <utility>

template <
    typename T1, typename T2,
    typename = typename std::enable_if<
        (std::is_base_of<
             std::input_iterator_tag,
             typename std::iterator_traits<T1>::iterator_category>::value ||
         std::is_base_of<
             std::output_iterator_tag,
             typename std::iterator_traits<T1>::iterator_category>::value) &&
        (std::is_base_of<
             std::input_iterator_tag,
             typename std::iterator_traits<T2>::iterator_category>::value ||
         std::is_base_of<
             std::output_iterator_tag,
             typename std::iterator_traits<T2>::iterator_category>::value) &&
        !std::is_same<T1, T2>::value && std::is_convertible<T2, T1>::value &&
        !std::is_pointer<T1>::value && !std::is_pointer<T2>::value>::type>
T1 f1(const T2 &a0) {
  return T1(a0);
}

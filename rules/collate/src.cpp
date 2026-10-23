#include <algorithm>
#include <ios>
#include <string>
#include <utility>

template <typename T1> using t1 = std::collate<T1>;

template <typename T1> using t2 = typename std::collate<T1>::char_type;

template <typename T1> using t3 = typename std::collate<T1>::string_type;

template <typename T1>
using t4 = typename std::collate<T1>::string_type::traits_type;

template <typename T1>
using t5 = typename std::collate<T1>::string_type::value_type;

template <typename T1>
using t6 = typename std::collate<T1>::string_type::allocator_type;

template <typename T1>
using t7 = typename std::collate<T1>::string_type::size_type;

template <typename T1>
using t8 = typename std::collate<T1>::string_type::difference_type;

template <typename T1>
using t9 = typename std::collate<T1>::string_type::reference;

template <typename T1>
using t10 = typename std::collate<T1>::string_type::const_reference;

template <typename T1>
using t11 = typename std::collate<T1>::string_type::pointer;

template <typename T1>
using t12 = typename std::collate<T1>::string_type::const_pointer;

template <typename T1> const std::locale::id &f1() {
  return std::collate<T1>::id;
}

#include <cstddef>
#include <ios>
#include <locale>
#include <string>
#include <utility>

template <typename T1> using t1 = std::messages<T1>;

template <typename T1> using t2 = typename std::messages<T1>::char_type;

template <typename T1> using t3 = typename std::messages<T1>::string_type;

template <typename T1> const std::locale::id &f1() {
  return std::messages<T1>::id;
}

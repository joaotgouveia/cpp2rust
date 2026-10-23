#include <algorithm>
#include <ios>
#include <string>
#include <utility>

template <typename T1> using t1 = std::collate<T1>;

template <typename T1> using t2 = typename std::collate<T1>::char_type;

template <typename T1> using t3 = typename std::collate<T1>::string_type;

template <typename T1>
int f1(const std::collate<T1> &a0, const T1 *a1, const T1 *a2, const T1 *a3,
       const T1 *a4) {
  return a0.compare(a1, a2, a3, a4);
}

template <typename T1>
std::basic_string<T1> f2(const std::collate<T1> &a0, const T1 *a1,
                         const T1 *a2) {
  return a0.transform(a1, a2);
}

template <typename T1>
long f3(const std::collate<T1> &a0, const T1 *a1, const T1 *a2) {
  return a0.hash(a1, a2);
}

template <typename T1> const std::locale::id &f4() {
  return std::collate<T1>::id;
}

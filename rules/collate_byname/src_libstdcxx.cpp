#include <algorithm>
#include <ios>
#include <string>
#include <utility>

template <typename T1> using t1 = std::collate_byname<T1>;

template <typename T1> using t2 = typename std::collate_byname<T1>::char_type;

template <typename T1> using t3 = typename std::collate_byname<T1>::string_type;

template <typename T1>
std::collate_byname<T1> f1(const char *a0, std::size_t a1) {
  return std::collate_byname<T1>(a0, a1);
}

template <typename T1>
std::collate_byname<T1> f2(const std::string &a0, std::size_t a1) {
  return std::collate_byname<T1>(a0, a1);
}

template <typename T1>
int f3(const std::collate_byname<T1> &a0, T1 const *a1, T1 const *a2,
       T1 const *a3, T1 const *a4) {
  return a0.compare(a1, a2, a3, a4);
}

template <typename T1>
std::basic_string<T1> f4(const std::collate_byname<T1> &a0, T1 const *a1,
                         T1 const *a2) {
  return a0.transform(a1, a2);
}

template <typename T1>
long f5(const std::collate_byname<T1> &a0, T1 const *a1, T1 const *a2) {
  return a0.hash(a1, a2);
}

#include <string>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
typename std::basic_string<T1, T2, T3>::size_type
f1(std::basic_string<T1, T2, T3> &a0, const T4 &a1) {
  return std::erase(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::basic_string<T1, T2, T3>::size_type
f2(std::basic_string<T1, T2, T3> &a0, T4 a1) {
  return std::erase_if(a0, std::move(a1));
}

template <typename T1, typename T2> using t1 = std::pmr::basic_string<T1, T2>;

using t2 = std::pmr::string;

using t3 = std::pmr::u16string;

using t4 = std::pmr::u32string;

using t5 = std::pmr::u8string;

using t6 = std::pmr::wstring;

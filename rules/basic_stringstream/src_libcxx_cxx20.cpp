#include <sstream>
#include <string>
#include <string_view>
#include <utility>

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3> f129(unsigned int a0, const T3 &a1) {
  return std::basic_stringstream<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3> f130(std::basic_string<T1, T2, T3> &&a0,
                                         unsigned int a1) {
  return std::basic_stringstream<T1, T2, T3>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_stringstream<T1, T2, T3>
f131(const std::basic_string<T1, T2, T4> &a0, unsigned int a1, const T3 &a2) {
  return std::basic_stringstream<T1, T2, T3>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_stringstream<T1, T2, T3>
f132(const std::basic_string<T1, T2, T4> &a0, unsigned int a1)
  requires((!std::is_same_v<
            T4, typename std::basic_stringstream<T1, T2, T3>::allocator_type>))
{
  return std::basic_stringstream<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<T1, T2, T4>
f133(const std::basic_stringstream<T1, T2, T3> &a0, const T4 &a1)
  requires(std::__is_allocator_v<T4>)
{
  return a0.template str<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
void f134(std::basic_stringstream<T1, T2, T3> &a0,
          const std::basic_string<T1, T2, T4> &a1) {
  return a0.template str<T4>(a1);
}

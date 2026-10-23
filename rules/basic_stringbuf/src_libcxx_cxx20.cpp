#include <sstream>
#include <string>
#include <string_view>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
std::basic_stringbuf<T1, T2, T3> f37(const std::basic_string<T1, T2, T4> &a0,
                                     std::ios_base::openmode a1)
  requires((!std::is_same_v<
            T4, typename std::basic_stringbuf<T1, T2, T3>::allocator_type>))
{
  return std::basic_stringbuf<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<T1, T2, T4> f38(const std::basic_stringbuf<T1, T2, T3> &a0,
                                  const T4 &a1)
  requires(std::__is_allocator_v<T4>)
{
  return a0.template str<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
void f39(std::basic_stringbuf<T1, T2, T3> &a0,
         const std::basic_string<T1, T2, T4> &a1)
  requires((!std::is_same_v<
            T4, typename std::basic_stringbuf<T1, T2, T3>::allocator_type>))
{
  return a0.template str<T4>(a1);
}

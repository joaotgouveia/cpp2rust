#include <string>
#include <utility>

template <typename T1>
bool f1(const std::fpos<T1> &a0, const std::fpos<T1> &a1) {
  return std::operator!= <T1>(a0, a1);
}

template <typename T1>
bool f2(const std::fpos<T1> &a0, const std::fpos<T1> &a1) {
  return std::operator== <T1>(a0, a1);
}

using t1 = std::streamoff;

using t2 = std::streampos;

using t3 = std::streamsize;

using t4 = std::u16streampos;

using t5 = std::u32streampos;

using t6 = std::wstreampos;

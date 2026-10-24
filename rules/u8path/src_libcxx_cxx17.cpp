#include <filesystem>
#include <utility>

template <
    typename T1,
    std::__enable_if_t<std::filesystem::__is_pathable<T1>::value, int> T2 = 0>
std::filesystem::path f1(const T1 &a0) {
  return std::filesystem::u8path<T1>(a0);
}

template <
    typename T1,
    std::__enable_if_t<std::filesystem::__is_pathable<T1>::value, int> T2 = 0>
std::filesystem::path f2(T1 a0, T1 a1) {
  return std::filesystem::u8path<T1>(std::move(a0), std::move(a1));
}

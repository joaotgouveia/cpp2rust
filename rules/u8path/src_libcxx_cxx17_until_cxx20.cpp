#include "../common/init.h"
#include "../common/traits.h"
#include <filesystem>
#include <utility>

template <typename T1> std::filesystem::path f1(const T1 &a0) {
  return std::filesystem::u8path<T1>(a0);
}

template <typename T1> std::filesystem::path f2(T1 a0, T1 a1) {
  return std::filesystem::u8path<T1>(a0, a1);
}

#include <expected>
#include <utility>

using t1 = std::bad_expected_access<void>;

const char *f1(const std::bad_expected_access<void> &a0) { return a0.what(); }

template <typename T1> using t2 = std::bad_expected_access<T1>;

template <typename T1> std::bad_expected_access<T1> f2(T1 a0) {
  return std::bad_expected_access<T1>(a0);
}

template <typename T1> T1 &f3(std::bad_expected_access<T1> &a0) {
  return a0.error();
}

template <typename T1> const T1 &f4(const std::bad_expected_access<T1> &a0) {
  return a0.error();
}

template <typename T1> T1 &&f5(std::bad_expected_access<T1> &&a0) {
  return std::move(a0).error();
}

template <typename T1> const T1 &&f6(const std::bad_expected_access<T1> &&a0) {
  return std::move(a0).error();
}

template <typename T1> const char *f7(const std::bad_expected_access<T1> &a0) {
  return a0.what();
}

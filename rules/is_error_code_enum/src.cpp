#include <system_error>
#include <utility>

template <typename T1> using t1 = std::is_error_code_enum<T1>;

template <typename T1> bool f1(const std::is_error_code_enum<T1> &a0) {
  return static_cast<bool>(a0);
}

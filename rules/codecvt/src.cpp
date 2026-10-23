#include <algorithm>
#include <codecvt>
#include <cstddef>
#include <ios>
#include <string>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

using t1 = std::codecvt<char, char, std::mbstate_t>;

using t2 = std::codecvt<char, char, __mbstate_t>::intern_type;

using t3 = std::codecvt<char, char, __mbstate_t>::extern_type;

using t4 = std::codecvt<char, char, __mbstate_t>::state_type;

int f1(const std::codecvt<char, char, std::mbstate_t> &a0) {
  return a0.encoding();
}

bool f2(const std::codecvt<char, char, std::mbstate_t> &a0) {
  return a0.always_noconv();
}

int f3(const std::codecvt<char, char, std::mbstate_t> &a0) {
  return a0.max_length();
}

const std::locale::id &f4() {
  return std::codecvt<char, char, __mbstate_t>::id;
}

using t5 = std::codecvt<wchar_t, char, std::mbstate_t>;

using t6 = std::codecvt<wchar_t, char, __mbstate_t>::intern_type;

using t7 = std::codecvt<wchar_t, char, __mbstate_t>::extern_type;

using t8 = std::codecvt<wchar_t, char, __mbstate_t>::state_type;

int f5(const std::codecvt<wchar_t, char, std::mbstate_t> &a0) {
  return a0.encoding();
}

bool f6(const std::codecvt<wchar_t, char, std::mbstate_t> &a0) {
  return a0.always_noconv();
}

int f7(const std::codecvt<wchar_t, char, std::mbstate_t> &a0) {
  return a0.max_length();
}

const std::locale::id &f8() {
  return std::codecvt<wchar_t, char, __mbstate_t>::id;
}

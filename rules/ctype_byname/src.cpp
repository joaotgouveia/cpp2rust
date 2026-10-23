#include <algorithm>
#include <ios>
#include <string>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

using t1 = std::ctype_byname<char>;

std::ctype<char>::char_type f1(const std::ctype_byname<char> &a0,
                               std::ctype<char>::char_type a1) {
  return a0.toupper(a1);
}

const std::ctype<char>::char_type *f2(const std::ctype_byname<char> &a0,
                                      std::ctype<char>::char_type *a1,
                                      const std::ctype<char>::char_type *a2) {
  return a0.toupper(a1, a2);
}

std::ctype<char>::char_type f3(const std::ctype_byname<char> &a0,
                               std::ctype<char>::char_type a1) {
  return a0.tolower(a1);
}

const std::ctype<char>::char_type *f4(const std::ctype_byname<char> &a0,
                                      std::ctype<char>::char_type *a1,
                                      const std::ctype<char>::char_type *a2) {
  return a0.tolower(a1, a2);
}

std::ctype<char>::char_type f5(const std::ctype_byname<char> &a0, char a1) {
  return a0.widen(a1);
}

const char *f6(const std::ctype_byname<char> &a0, const char *a1,
               const char *a2, std::ctype<char>::char_type *a3) {
  return a0.widen(a1, a2, a3);
}

char f7(const std::ctype_byname<char> &a0, std::ctype<char>::char_type a1,
        char a2) {
  return a0.narrow(a1, a2);
}

const std::ctype_base::mask *f8(const std::ctype_byname<char> &a0) {
  return a0.table();
}

const std::ctype_base::mask *f9() { return std::ctype<char>::classic_table(); }

using t2 = std::ctype_byname<wchar_t>;

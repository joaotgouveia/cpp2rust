#include "../common/init.h"
#include "../common/traits.h"
#include <ios>
#include <utility>

using t13 = std::collate_byname<char>;

using t14 = std::collate_byname<char>::char_type;

using t15 = std::collate_byname<char>::string_type;

int f85(const std::collate_byname<char> &a0,
        const std::collate<char>::char_type *a1,
        const std::collate<char>::char_type *a2,
        const std::collate<char>::char_type *a3,
        const std::collate<char>::char_type *a4) {
  return a0.compare(a1, a2, a3, a4);
}

std::collate<char>::string_type f86(const std::collate_byname<char> &a0,
                                    const std::collate<char>::char_type *a1,
                                    const std::collate<char>::char_type *a2) {
  return a0.transform(a1, a2);
}

long f87(const std::collate_byname<char> &a0,
         const std::collate<char>::char_type *a1,
         const std::collate<char>::char_type *a2) {
  return a0.hash(a1, a2);
}

long f88(const std::collate_byname<char> &a0) { return a0.use_count(); }

using t16 = std::collate_byname<wchar_t>;

using t17 = std::collate_byname<wchar_t>::char_type;

using t18 = std::collate_byname<wchar_t>::string_type;

int f89(const std::collate_byname<wchar_t> &a0,
        const std::collate<wchar_t>::char_type *a1,
        const std::collate<wchar_t>::char_type *a2,
        const std::collate<wchar_t>::char_type *a3,
        const std::collate<wchar_t>::char_type *a4) {
  return a0.compare(a1, a2, a3, a4);
}

std::collate<wchar_t>::string_type
f90(const std::collate_byname<wchar_t> &a0,
    const std::collate<wchar_t>::char_type *a1,
    const std::collate<wchar_t>::char_type *a2) {
  return a0.transform(a1, a2);
}

long f91(const std::collate_byname<wchar_t> &a0,
         const std::collate<wchar_t>::char_type *a1,
         const std::collate<wchar_t>::char_type *a2) {
  return a0.hash(a1, a2);
}

long f92(const std::collate_byname<wchar_t> &a0) { return a0.use_count(); }

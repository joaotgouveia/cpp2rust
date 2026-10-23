#include <ios>
#include <utility>

using t4 = std::collate_byname<char>;

using t5 = std::collate_byname<char>::char_type;

using t6 = std::collate_byname<char>::string_type;

int f6(const std::collate_byname<char> &a0,
       const std::collate<char>::char_type *a1,
       const std::collate<char>::char_type *a2,
       const std::collate<char>::char_type *a3,
       const std::collate<char>::char_type *a4) {
  return a0.compare(a1, a2, a3, a4);
}

std::collate<char>::string_type f7(const std::collate_byname<char> &a0,
                                   const std::collate<char>::char_type *a1,
                                   const std::collate<char>::char_type *a2) {
  return a0.transform(a1, a2);
}

long f8(const std::collate_byname<char> &a0,
        const std::collate<char>::char_type *a1,
        const std::collate<char>::char_type *a2) {
  return a0.hash(a1, a2);
}

long f9(const std::collate_byname<char> &a0) { return a0.use_count(); }

using t7 = std::collate_byname<wchar_t>;

using t8 = std::collate_byname<wchar_t>::char_type;

using t9 = std::collate_byname<wchar_t>::string_type;

int f10(const std::collate_byname<wchar_t> &a0,
        const std::collate<wchar_t>::char_type *a1,
        const std::collate<wchar_t>::char_type *a2,
        const std::collate<wchar_t>::char_type *a3,
        const std::collate<wchar_t>::char_type *a4) {
  return a0.compare(a1, a2, a3, a4);
}

std::collate<wchar_t>::string_type
f11(const std::collate_byname<wchar_t> &a0,
    const std::collate<wchar_t>::char_type *a1,
    const std::collate<wchar_t>::char_type *a2) {
  return a0.transform(a1, a2);
}

long f12(const std::collate_byname<wchar_t> &a0,
         const std::collate<wchar_t>::char_type *a1,
         const std::collate<wchar_t>::char_type *a2) {
  return a0.hash(a1, a2);
}

long f13(const std::collate_byname<wchar_t> &a0) { return a0.use_count(); }

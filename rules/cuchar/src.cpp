#include "../common/init.h"
#include "../common/traits.h"
#include <cuchar>
#include <cwchar>
#include <stdlib.h>
#include <utility>

size_t f1(char *__restrict a0, char16_t a1, mbstate_t *__restrict a2) {
  return c16rtomb(a0, a1, a2);
}

size_t f2(char *__restrict a0, char32_t a1, mbstate_t *__restrict a2) {
  return c32rtomb(a0, a1, a2);
}

size_t f3(char *__restrict a0, char8_t a1, mbstate_t *__restrict a2) {
  return c8rtomb(a0, a1, a2);
}

size_t f4(char16_t *__restrict a0, const char *__restrict a1, size_t a2,
          mbstate_t *__restrict a3) {
  return mbrtoc16(a0, a1, a2, a3);
}

size_t f5(char32_t *__restrict a0, const char *__restrict a1, size_t a2,
          mbstate_t *__restrict a3) {
  return mbrtoc32(a0, a1, a2, a3);
}

size_t f6(char8_t *__restrict a0, const char *__restrict a1, size_t a2,
          mbstate_t *__restrict a3) {
  return mbrtoc8(a0, a1, a2, a3);
}

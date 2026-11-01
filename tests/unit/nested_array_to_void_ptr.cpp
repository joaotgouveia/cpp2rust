#include <cassert>
#include <cstring>

static int first_int(const void *p) { return *static_cast<const int *>(p); }

static char first_char(const void *p) {
  return *static_cast<const char *>(p);
}

int main() {
  int rows[2][3] = {{1, 2, 3}, {4, 5, 6}};
  int (*p)[3] = rows;
  assert(first_int(rows) == 1);
  assert(first_int(p) == 1);
  assert(std::memcmp(rows, rows[0], sizeof(rows[0])) == 0);
  ++p;
  assert(first_int(p) == 4);
  char words[3][4] = {"cat", "ant", "bee"};
  assert(first_char(words) == 'c');
  assert(std::memcmp(words, "cat", 4) == 0);
  return 0;
}

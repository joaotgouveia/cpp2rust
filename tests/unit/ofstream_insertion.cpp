#include <cassert>
#include <cstdio>
#include <fstream>

int main() {
  {
    std::ofstream out("./ofstream_insertion.txt", std::ios_base::out);
    out << 42;
    out << " items";
  }
  FILE *f = fopen("./ofstream_insertion.txt", "rb");
  char buf[16] = {0};
  size_t n = fread(buf, 1, sizeof(buf), f);
  fclose(f);
  assert(n == 8);
  assert(buf[0] == '4' && buf[1] == '2' && buf[2] == ' ' && buf[7] == 's');
  return 0;
}

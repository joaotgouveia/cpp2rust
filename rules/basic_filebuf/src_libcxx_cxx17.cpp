#include <filesystem>
#include <fstream>
#include <ios>
#include <string.h>
#include <utility>

template <typename T1, typename T2>
std::basic_filebuf<T1, T2> *f24(std::basic_filebuf<T1, T2> &a0,
                                const std::filesystem::path &a1,
                                std::ios_base::openmode a2) {
  return a0.open(a1, a2);
}

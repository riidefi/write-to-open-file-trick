#include <fstream>
#include <string>
#include <utility>
#include <stdint.h>
#include <stddef.h>

extern "C" {
  void WriteToOpenFileC(const char* path, size_t path_len, const char* data, size_t data_len);
}

// Avoiding string_view, span...

static void WriteToOpenFile(std::string&& path, const char* data, size_t data_len) {
  std::ofstream file;
  file.open(std::move(path), std::ios_base::binary | std::ios_base::out);
  file.write(data, data_len);
  file.close();
}

extern "C" void WriteToOpenFileC(const char* path, uint64_t path_len, const char* data, uint64_t data_len) {
  WriteToOpenFile(std::string(path, path_len), data, data_len);
}

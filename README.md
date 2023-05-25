```cpp
//
// I don't know why this works, but it does.
// Perhaps since Rust's fs lib offers many more safety guarantees.
// In particular, this is necessary for .dll files open by the main app.
// Rust's standard library will refuse to write to them on Windows.
//
static void WriteToOpenFile(std::string_view path,
                            std::span<const char> dumped) {
  std::ofstream wFile;
  wFile.open(std::string(path), std::ios_base::binary | std::ios_base::out);
  wFile.write(dumped.data(), dumped.size());
  wFile.close();
}

void ExtractZip(std::string from_file, std::string to_folder) {
  (void)c_rsl_extract_zip(
      from_file.c_str(), to_folder.c_str(),
      +[](const char* path, const void* buf, uint32_t len) {
        WriteToOpenFile(path, {reinterpret_cast<const char*>(buf),
                               static_cast<size_t>(len)});
      });
}
```
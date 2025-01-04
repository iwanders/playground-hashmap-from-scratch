#include <unordered_map>
#include <cstdint>

struct Wrapper {
  std::unordered_map<std::uint64_t, std::uint64_t> map;
};

extern "C" {
  Wrapper* um_u64_u64_create() {
    return new Wrapper;
  }
  void um_u64_u64_free(Wrapper* w) {
    free(w);
  }

  bool um_u64_u64_is_empty(Wrapper* w) {
    return w->map.empty();
  }

  void um_u64_u64_insert(Wrapper* w, std::uint64_t key, std::uint64_t value) {
    w->map[key] = value;
  }

  bool um_u64_u64_contains(Wrapper* w, std::uint64_t key) {
    return w->map.find(key) != w->map.end();
  }

  std::uint64_t um_u64_u64_get(Wrapper* w, std::uint64_t key) {
    return w->map.find(key)->second;
  }

  void um_u64_u64_remove(Wrapper* w, std::uint64_t key) {
    w->map.erase(key);
  }
  std::uint64_t um_u64_u64_remove_return(Wrapper* w, std::uint64_t key) {
    const auto it = w->map.find(key);
    const auto v = it->second;
    w->map.erase(it);
    return v;
  }
  

  std::uint64_t um_u64_u64_len(Wrapper* w) {
    return w->map.size();
  }
}

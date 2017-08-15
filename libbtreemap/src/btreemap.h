void* btreemap_new(void);
bool btreemap_contains_key(*void, uint32_t);
uint32_t btreemap_insert(void*, uint32_t, uint32_t, *void*);
uint32_t btreemap_keys(void*, void*);
uint32_t btreemap_values(void*, void*);
uint32_t btreemap_remove(void*, uint32_t, *void);
uint32_t btreemap_len(void*);
uint32_t btreemap_is_empty(void*);

#include <assert.h>
#include <dlfcn.h> // perhaps "dynamic loading functions"
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

typedef void (*greet_t)(const char* name);
typedef void (*woops_t)();

void
print_mapping_count()
{
  const size_t buf_size = 1024;
  char buf[buf_size];
  printf("mapping count: ");
  fflush(stdout);
  snprintf(buf,
           buf_size,
           "bash -c 'cat /proc/%d/maps | grep libgreet | wc -l'",
           getpid());
  system(buf);
}

int
main(int argc, char** argv)
{
  (void)argc;
  (void)argv;

  print_mapping_count();

  printf("> dlopen(libgreet-rs, RTLD_NOW)\n");
  void* lib = dlopen("./libgreet-rs/target/debug/libgreet.so", RTLD_NOW);
  assert(lib);
  print_mapping_count();

  printf("> dlclose(libgreet-rs), will it work?\n");
  dlclose(lib);
  print_mapping_count();

  return 0;
}


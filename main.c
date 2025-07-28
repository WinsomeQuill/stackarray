#include "stackarray.h"
#include <stdio.h>

int main(int argc, char *argv[]) {
  Stack s;

  void *buffer[8];

  init_stack(&s, &buffer);

  int a = 1;

  push(&s, "Hello");
  push(&s, "Hello1");
  push(&s, "Hello2");

  printf("Get element 2: %s\n", (char *)get(&s, 1));

  while (!is_empty(&s)) {
    printf("Popped: %s\n", (char *)pop(&s));
  }

  return 0;
}

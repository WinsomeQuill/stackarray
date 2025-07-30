#include "stackarray.h"
#include <stdio.h>
#include <stdlib.h>

void init_stack(Stack *s, void *buffer, int max_size) {
  s->sp = -1;
  s->buffer = (void **)buffer;
  s->max_size = max_size;
}

int is_empty(Stack *s) { return s->sp == -1; }

int is_full(Stack *s) { return s->sp >= s->max_size - 1; }

void *get(Stack *s, int index) {
  if (is_empty(s)) {
    printf("Stack underflow!\n");
    return NULL;
  }
  if (index < 0 || index > s->sp) {
    printf("Index out of bounds!\n");
    return NULL;
  }

  return s->buffer[index];
}

int push(Stack *s, void *value) {
  if (is_full(s)) {
    printf("Stack overflow!\n");
    return 0;
  }

  s->buffer[++s->sp] = value;
  return 1;
}

void *pop(Stack *s) {
  if (is_empty(s)) {
    printf("Stack underflow!\n");
    return 0;
  }

  return s->buffer[s->sp--];
}

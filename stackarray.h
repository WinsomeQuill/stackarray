typedef struct {
  // Ссылка на буффер стека
  void **buffer;
  // Stack Pointer
  int sp;
  // Максимальный размер стека
  int max_size;
} Stack;

void init_stack(Stack *s, void *buffer, int max_size);
int is_empty(Stack *s);
int is_full(Stack *s);
void *get(Stack *s, int index);
int push(Stack *s, void *value);
void *pop(Stack *s);

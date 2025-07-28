typedef struct {
  // Ссылка на буффер стека
  int **buffer;
  // Stack Pointer
  int sp;
} Stack;

void init_stack(Stack *s, void *buffer);
int is_empty(Stack *s);
int is_full(Stack *s);
void *get(Stack *s, int index);
int push(Stack *s, void *value);
void *pop(Stack *s);

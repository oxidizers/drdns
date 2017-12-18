#ifndef ALLOC_H
#define ALLOC_H

extern char *alloc(unsigned int n);
extern void alloc_free(char *x);
int alloc_re(char **x, unsigned int m,unsigned int n);

#endif

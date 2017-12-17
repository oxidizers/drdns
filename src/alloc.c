#include <stdlib.h>
#include "alloc.h"
#include "error.h"

/* SKIPPED: corrode issues
#define ALIGNMENT 16
#define SPACE 2048

typedef union { char irrelevant[ALIGNMENT]; double d; } aligned;
static aligned realspace[SPACE / ALIGNMENT];
#define space ((char *) realspace)
static unsigned int avail = SPACE;
*/

char *alloc(unsigned int n)
{
  char *x;
  /* SKIPPED: corrode issues
  n = ALIGNMENT + n - (n & (ALIGNMENT - 1));
  if (n <= avail) { avail -= n; return space + avail; }
  */
  x = malloc(n);
  if (!x) errno = error_nomem;
  return x;
}

void alloc_free(char *x)
{
  /* SKIPPED: corrode issues
  if (x >= space)
    if (x < space + SPACE)
      return;
  */
  free(x);
}

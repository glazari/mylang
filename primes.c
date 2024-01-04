#include <stdio.h>

int isPrime(int n) {
  int i;
  for (i = 2; i < n; i++) {
    if (n % i == 0) {
      return 0;
    }
  }
  return 1;
}

int main() {
  int n = 2;
  for (n = 2; n < 100000; n++) {
    if (isPrime(n)) {
      printf("%d\n", n);
    }
  }
}

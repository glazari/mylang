
clean:
	rm -f *.o *.a *.so *.out *.gch *.exe


ccompile:
	gcc -O0 primes.c
	gcc -O0 -S -masm=intel primes.c

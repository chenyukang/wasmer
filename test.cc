#include <stdio.h>
struct A { ~A(); };
struct B { ~B(); };
void foo() { throw 0xB612; }
void bar() { B b; foo(); }
void qux() { try { A a; bar(); } catch (int x) { puts(""); } }

int main()
{
	puts("hello world");
	return 0;
}


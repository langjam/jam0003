#define f0(g) char g;
#define f1(g) f0(g##0) f0(g##1) f0(g##2) f0(g##3) f0(g##4) f0(g##5) f0(g##6) f0(g##7) f0(g##8) f0(g##9)
#define f2(g) f1(g##0) f1(g##1) f1(g##2) f1(g##3) f1(g##4) f1(g##5) f1(g##6) f1(g##7) f1(g##8) f1(g##9)
#define f3(g) f2(g##0) f2(g##1) f2(g##2) f2(g##3) f2(g##4) f2(g##5) f2(g##6) f2(g##7) f2(g##8) f2(g##9)
#define f4(g) f3(g##0) f3(g##1) f3(g##2) f3(g##3) f3(g##4) f3(g##5) f3(g##6) f3(g##7) f3(g##8) f3(g##9)
#define f5(g) f4(g##0) f4(g##1) f4(g##2) f4(g##3) f4(g##4) f4(g##5) f4(g##6)
f5(g)

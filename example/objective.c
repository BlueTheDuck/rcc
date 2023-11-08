// My objective is parsing this file :)

typedef unsigned int uint32_t;

void main() {
    uint32_t x, *p, y;
    p = &x;
    if(x == 0) {
        x += 1;
    } else {
        x += 2;
    }
    y = *p;
}

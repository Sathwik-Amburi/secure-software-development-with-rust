#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

int main() {
    FILE* file;
    char ch;

    while (1) {
        // Open the file
        file = fopen("test.txt", "r");

        if (file == NULL) {
            printf("Error: Could not open file\n");
            exit(1);
        }

        // Delay for 10 seconds
        sleep(1);
    }

    return 0;
}

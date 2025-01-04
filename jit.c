#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/mman.h>
#include <unistd.h>

// Define a function pointer type for the function to be called from the mmap'ed region
typedef int (*func_ptr)();

int main() {
    // Size of the memory region to map (enough to hold the code)
    size_t size = 1024;

    // Allocate memory with mmap for executable code
    void *mem = mmap(NULL, size, PROT_READ | PROT_WRITE | PROT_EXEC, MAP_ANON | MAP_PRIVATE, -1, 0);
    if (mem == MAP_FAILED) {
        perror("mmap failed");
        return 1;
    }

    // Example x86 assembly instructions (this is just a simple example, e.g., adding 2 + 3)
    unsigned char code[] = {
        0xB8, 0x02, 0x00, 0x00, 0x00,  // mov eax, 2
        0x03, 0xC0,                    // add eax, eax (eax = eax + eax)
        0xC3                           // ret (return)
    };

    // Copy the code to the memory-mapped region
    memcpy(mem, code, sizeof(code));

    // Cast the memory region to a function pointer
    func_ptr func = (func_ptr)mem;

    // Call the function and retrieve the result in eax
    int result = func();  // eax will be returned as the result

    // Print the result (should be 4 in this case, since 2 + 2 = 4)
    printf("Result: %d\n", result);

    // Unmap the memory region when done
    munmap(mem, size);

    return 0;
}

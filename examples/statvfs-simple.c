#include <stdio.h>
#include <stdlib.h>
#include <sys/statvfs.h>

int main(int argc, char *argv[]) {
    struct statvfs fs;
    if (argc < 2) {
        fprintf(stderr, "Usage: %s <path>\n", argv[0]);
        exit(EXIT_FAILURE);
    }
    if (statvfs(argv[1], &fs) == -1) {
        perror("statvfs");
        exit(EXIT_FAILURE);
    }
    printf("Block size(f_bsize): %lu bytes\n", fs.f_bsize);
    printf("Block size(f_frsize): %lu bytes\n", fs.f_frsize);
    printf("Total blocks: %lu\n", fs.f_blocks);
    printf("Free blocks: %lu\n", fs.f_bfree);
    printf("Available blocks: %lu\n", fs.f_bavail);
    printf("Inodes: %lu\n", fs.f_files);
    printf("Free inodes: %lu\n", fs.f_ffree);
    printf("Avail inodes: %lu\n", fs.f_favail);
    printf("Name max length: %lu\n", fs.f_namemax);
    return 0;
}

#include <stdio.h>
#include <stdlib.h>
#include <sys/mount.h>
//#include <sys/statfs.h> //linux才需要，macos不需要

int main() {
    const char *path = "/";

    struct statfs fs_info;

    // 调用 statfs 函数
    int result = statfs(path, &fs_info);

    if (result == 0) {
        // 获取块大小
        printf("Block Size: %lu bytes\n", fs_info.f_bsize);

        printf("f_type: %lu \n", fs_info.f_type);
        printf("f_bsize: %lu \n", fs_info.f_bsize);
        printf("f_blocks: %lu \n", fs_info.f_blocks);
        printf("f_bfree: %lu \n", fs_info.f_bfree);
        printf("f_bavail: %lu \n", fs_info.f_bavail);
        printf("f_files: %lu \n", fs_info.f_files);
        printf("f_ffree: %lu \n", fs_info.f_ffree);
        printf("f_fsid: %lu \n", fs_info.f_fsid);
        printf("f_flags: %lu \n", fs_info.f_flags);

    } else {
        // 错误处理
        perror("Error calling statfs");
        return EXIT_FAILURE;
    }

    return EXIT_SUCCESS;
}





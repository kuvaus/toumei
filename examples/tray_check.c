//In the main folder:
//cargo build --release --features capi
//gcc ./examples/tray_check.c -ltoumei -L/target/release -I/include

#include <stdio.h>
#include "toumei.h"

int main() {
    ToumeiMode mode;
    ToumeiResult result = detect_tray_transparency(&mode);

    if (result == Ok) {
        if (mode == Transparent) { printf("🎉 System tray transparency is enabled!\n"); }
        if (mode == Opaque)      { printf("🔒 System tray transparency is disabled\n"); }
    } else {
        // Handle error case
        printf("Error: ");
        // ... error handling code ...
        switch(result) {
            case ErrUnsupportedPlatform:
                printf("Unsupported platform");
                break;
            case ErrWindowNotFound:
                printf("Window not found");
                break;
            default:
                printf("Unknown error");
        }
    }
    
    return 0;
}

// This file was generated by gir (https://github.com/gtk-rs/gir)
// from 
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#include "manual.h"
#include <stdio.h>

#define PRINT_CONSTANT(CONSTANT_NAME) \
    printf("%s;", #CONSTANT_NAME); \
    printf(_Generic((CONSTANT_NAME), \
                    char *: "%s", \
                    const char *: "%s", \
                    char: "%c", \
                    signed char: "%hhd", \
                    unsigned char: "%hhu", \
                    short int: "%hd", \
                    unsigned short int: "%hu", \
                    int: "%d", \
                    unsigned int: "%u", \
                    long: "%ld", \
                    unsigned long: "%lu", \
                    long long: "%lld", \
                    unsigned long long: "%llu", \
                    float: "%f", \
                    double: "%f", \
                    long double: "%ld"), \
           CONSTANT_NAME); \
    printf("\n");

int main() {
    PRINT_CONSTANT((gint) ALSACTL_CARD_ERROR_DISCONNECTED);
    PRINT_CONSTANT((gint) ALSACTL_CARD_ERROR_ELEM_EXIST);
    PRINT_CONSTANT((gint) ALSACTL_CARD_ERROR_ELEM_NOT_FOUND);
    PRINT_CONSTANT((gint) ALSACTL_CARD_ERROR_ELEM_NOT_SUPPORTED);
    PRINT_CONSTANT((gint) ALSACTL_CARD_ERROR_ELEM_OWNED);
    PRINT_CONSTANT((gint) ALSACTL_CARD_ERROR_FAILED);
    PRINT_CONSTANT((guint) ALSACTL_ELEM_ACCESS_FLAG_INACTIVE);
    PRINT_CONSTANT((guint) ALSACTL_ELEM_ACCESS_FLAG_LOCK);
    PRINT_CONSTANT((guint) ALSACTL_ELEM_ACCESS_FLAG_OWNER);
    PRINT_CONSTANT((guint) ALSACTL_ELEM_ACCESS_FLAG_READ);
    PRINT_CONSTANT((guint) ALSACTL_ELEM_ACCESS_FLAG_TLV_CALLBACK);
    PRINT_CONSTANT((guint) ALSACTL_ELEM_ACCESS_FLAG_TLV_COMMAND);
    PRINT_CONSTANT((guint) ALSACTL_ELEM_ACCESS_FLAG_TLV_READ);
    PRINT_CONSTANT((guint) ALSACTL_ELEM_ACCESS_FLAG_TLV_WRITE);
    PRINT_CONSTANT((guint) ALSACTL_ELEM_ACCESS_FLAG_USER);
    PRINT_CONSTANT((guint) ALSACTL_ELEM_ACCESS_FLAG_VOLATILE);
    PRINT_CONSTANT((guint) ALSACTL_ELEM_ACCESS_FLAG_WRITE);
    PRINT_CONSTANT((guint) ALSACTL_ELEM_EVENT_MASK_ADD);
    PRINT_CONSTANT((guint) ALSACTL_ELEM_EVENT_MASK_INFO);
    PRINT_CONSTANT((guint) ALSACTL_ELEM_EVENT_MASK_REMOVE);
    PRINT_CONSTANT((guint) ALSACTL_ELEM_EVENT_MASK_TLV);
    PRINT_CONSTANT((guint) ALSACTL_ELEM_EVENT_MASK_VALUE);
    PRINT_CONSTANT((gint) ALSACTL_ELEM_IFACE_TYPE_CARD);
    PRINT_CONSTANT((gint) ALSACTL_ELEM_IFACE_TYPE_HWDEP);
    PRINT_CONSTANT((gint) ALSACTL_ELEM_IFACE_TYPE_MIXER);
    PRINT_CONSTANT((gint) ALSACTL_ELEM_IFACE_TYPE_PCM);
    PRINT_CONSTANT((gint) ALSACTL_ELEM_IFACE_TYPE_RAWMIDI);
    PRINT_CONSTANT((gint) ALSACTL_ELEM_IFACE_TYPE_SEQUENCER);
    PRINT_CONSTANT((gint) ALSACTL_ELEM_IFACE_TYPE_TIMER);
    PRINT_CONSTANT((gint) ALSACTL_ELEM_TYPE_BOOLEAN);
    PRINT_CONSTANT((gint) ALSACTL_ELEM_TYPE_BYTES);
    PRINT_CONSTANT((gint) ALSACTL_ELEM_TYPE_ENUMERATED);
    PRINT_CONSTANT((gint) ALSACTL_ELEM_TYPE_IEC60958);
    PRINT_CONSTANT((gint) ALSACTL_ELEM_TYPE_INTEGER);
    PRINT_CONSTANT((gint) ALSACTL_ELEM_TYPE_INTEGER64);
    PRINT_CONSTANT((gint) ALSACTL_ELEM_TYPE_NONE);
    PRINT_CONSTANT((gint) ALSACTL_EVENT_TYPE_ELEM);
    return 0;
}

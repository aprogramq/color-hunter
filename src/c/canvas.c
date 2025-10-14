#include <cairo/cairo-svg.h>
#include <cairo/cairo.h>
#include <dirent.h>
#include <pwd.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#define PLACEHOLDER 2
#define MAX_COUNT_PALETTE 1000

struct RGB {
    double r;
    double g;
    double b;
};

void save_palette(int count, char **hex);
char *alloc_cstring(const char *js_string);
int hex_to_int(const char *hex);
struct RGB hexToRGB(const char *hex);

char *alloc_cstring(const char *js_string) { return strdup(js_string); }
char *path_to_save();

// int main() {
//     char *color_palette[3] = {"111111", "222222", "333333"};
//     save_palette(3, color_palette);
// }

void save_palette(int count, char **hex) {
    // for (int i = 0; i < count; i++) {
    //   printf("hex[%d] address: %p, content: %s\n", i, (void *)hex[i],
    //   hex[i]);
    // }

    char *path_to_palette = path_to_save();
    cairo_surface_t *surface =
        cairo_svg_surface_create(path_to_palette, 500, 500);
    cairo_t *cr = cairo_create(surface);
    struct RGB colors[count];

    for (int i = 0; i < count; i++) {
        colors[i] = hexToRGB(hex[i]);
    }
    int x_position = 60;

    for (int i = 0; i < count; i++) {
        char *hex_color = calloc(8, sizeof(char));
        *hex_color = '#';
        // printf("%f, %f, %f\n", colors[i].r, colors[i].g, colors[i].b);
        cairo_set_source_rgb(cr, colors[i].r / 255.0f, colors[i].g / 255.0f,
                             colors[i].b / 255.0f);
        cairo_rectangle(cr, 169 * i, 150, 170, 200);
        cairo_fill(cr);

        cairo_font_face_t *font = cairo_toy_font_face_create(
            "Iosevka", CAIRO_FONT_SLANT_NORMAL, CAIRO_FONT_WEIGHT_NORMAL);
        cairo_set_font_face(cr, font);
        cairo_set_font_size(cr, 14.0f);
        cairo_set_source_rgb(cr, 1, 1, 1);
        cairo_move_to(cr, (i + 1) * x_position, 250);
        if (i < 1)
            x_position += 53;
        else
            x_position += 19;

        cairo_show_text(cr, strcat(hex_color, hex[i]));
        free(hex_color);
    }

    cairo_destroy(cr);
    cairo_surface_destroy(surface);
    free(path_to_palette);
}

char *path_to_save() {
    char path_template[] = "/home/%s/Pictures/colors/";
    uid_t uid = getuid();
    struct passwd *pw = getpwuid(uid);

    char *path_to_palette =
        malloc((strlen(path_template) - PLACEHOLDER + strlen(pw->pw_name) + 1) *
               sizeof(char));
    sprintf(path_to_palette, path_template, pw->pw_name);

    DIR *palette_dir = opendir(path_to_palette);
    struct dirent *entry;
    int palette_number = 0;
    char name[MAX_COUNT_PALETTE + 9] = "palette_%d.svg";
    while ((entry = readdir(palette_dir)) != NULL) {
        if (strncmp(entry->d_name, "palette", 7) == 0)
            palette_number += 1;
    }
    sprintf(name, name, palette_number);

    char *file_name = calloc(1, (strlen("palette_%d.svg") + MAX_COUNT_PALETTE));

    if (palette_number > 0) {
        strcpy(file_name, name);
    } else {
        strcpy(file_name, "palette.svg");
    }
    strcat(path_to_palette, file_name);
    return path_to_palette;
}

struct RGB hexToRGB(const char *hex) {
    char cleanHex[7];
    if (hex[0] == '#') {
        hex++;
    }

    strncpy(cleanHex, hex, 6);
    cleanHex[6] = '\0';

    struct RGB rgb;
    rgb.r = hex_to_int(cleanHex);
    rgb.g = hex_to_int(cleanHex + 2);
    rgb.b = hex_to_int(cleanHex + 4);
    return rgb;
}

int hex_to_int(const char *hex) {
    int value;
    sscanf(hex, "%2x", &value);
    return value;
}

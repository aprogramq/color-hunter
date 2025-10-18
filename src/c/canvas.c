#include "./iosevka_data.h"
#include <cairo/cairo-ft.h>
#include <cairo/cairo-svg.h>
#include <cairo/cairo.h>
#include <dirent.h>
#include <fontconfig/fontconfig.h>
#include <freetype2/freetype/freetype.h>
#include <freetype2/ft2build.h>
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

void save_palette(int count, char **hex, char **text_colors);
char *alloc_cstring(const char *js_string);
int hex_to_int(const char *hex);
struct RGB hexToRGB(const char *hex);

char *alloc_cstring(const char *js_string) { return strdup(js_string); }
char *path_to_save();

int main() {
    char *color_palette[3] = {"111111", "222222", "333333"};
    save_palette(3, color_palette, color_palette);
}

void save_palette(int count, char **hex, char **text_colors) {
    FT_Library library;
    FT_Face face;
    FT_Init_FreeType(&library);
    FT_New_Memory_Face(library, IosevkaSS01_Medium_ttf,
                       IosevkaSS01_Medium_ttf_len, 0, &face);

    char *path_to_palette = path_to_save();
    cairo_surface_t *surface =
        cairo_svg_surface_create(path_to_palette, 500, 500);
    cairo_t *cr = cairo_create(surface);

    struct RGB colors[count];
    struct RGB text_rgb[count];
    for (int i = 0; i < count; i++) {
        colors[i] = hexToRGB(hex[i]);
        text_rgb[i] = hexToRGB(text_colors[i]);
    }
    printf("\t%f, %f, %f\n", text_rgb[0].r, text_rgb[0].g, text_rgb[0].b);

    int x_position = 60;

    for (int i = 0; i < count; i++) {
        char *hex_color = (char *)calloc(8, sizeof(char));
        *hex_color = '#';
        cairo_set_source_rgb(cr, colors[i].r / 255.0f, colors[i].g / 255.0f,
                             colors[i].b / 255.0f);
        cairo_rectangle(cr, 169 * i, 150, 170, 200);
        cairo_fill(cr);

        cairo_font_face_t *font =
            cairo_ft_font_face_create_for_ft_face(face, 0);
        cairo_set_font_face(cr, font);
        cairo_set_font_size(cr, 14.0f);
        cairo_set_source_rgb(cr, text_rgb[i].r / 255.0f, text_rgb[i].g / 255.0f,
                             text_rgb[i].b / 255.0f);
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

    char *path_to_palette = (char *)malloc(
        (strlen(path_template) - PLACEHOLDER + strlen(pw->pw_name) + 1) *
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

    char *file_name =
        (char *)calloc(1, (strlen("palette_%d.svg") + MAX_COUNT_PALETTE));

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

#include "./iosevka_font.h"
#include <cairo/cairo-ft.h>
#include <cairo/cairo-svg.h>
#include <cairo/cairo.h>
#include <dirent.h>
#include <fontconfig/fontconfig.h>
#include <freetype2/freetype/freetype.h>
#include <freetype2/ft2build.h>
#include <pwd.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <strings.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <unistd.h>
#define PLACEHOLDER 2
#define MAX_COUNT_PALETTE 1000
#define DOUBLE_QUOUTE 34
#define SAVE_PATH 1

struct RGB {
    double r;
    double g;
    double b;
};
int save_palette(int count, char **hex, char **text_colors);
char *alloc_cstring(const char *js_string);
int hex_to_int(const char *hex);
struct RGB hexToRGB(const char *hex);

char *alloc_cstring(const char *js_string) { return strdup(js_string); }
char *path_to_save();

int save_palette(int count, char **hex, char **text_colors) {
    FT_Library library;
    FT_Face face;
    FT_Init_FreeType(&library);
    FT_New_Memory_Face(library, iosevka_font, iosevka_font_len, 0, &face);

    char *path_to_palette = path_to_save();
    cairo_surface_t *surface = cairo_svg_surface_create(path_to_palette, 500, 500);
    cairo_t *cr = cairo_create(surface);

    struct RGB colors[count];
    struct RGB text_rgb[count];
    for (int i = 0; i < count; i++) {
        colors[i] = hexToRGB(hex[i]);
        text_rgb[i] = hexToRGB(text_colors[i]);
    }

    int x_position = 60;

    for (int i = 0; i < count; i++) {
        char *hex_color = (char *)calloc(8, sizeof(char));
        *hex_color = '#';
        cairo_set_source_rgb(cr, colors[i].r / 255.0f, colors[i].g / 255.0f, colors[i].b / 255.0f);
        cairo_rectangle(cr, 169 * i, 150, 170, 200);
        cairo_fill(cr);

        cairo_font_face_t *font = cairo_ft_font_face_create_for_ft_face(face, 0);
        cairo_set_font_face(cr, font);
        cairo_set_font_size(cr, 14.0f);
        cairo_set_source_rgb(cr, text_rgb[i].r / 255.0f, text_rgb[i].g / 255.0f, text_rgb[i].b / 255.0f);
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
	return 1;
}

char **get_setting_path();
char *path_to_save() {
    char **settings = get_setting_path();
    DIR *palette_dir = opendir(settings[SAVE_PATH]);
    if (palette_dir == NULL) {
        mkdir(settings[SAVE_PATH], 0700);
        palette_dir = opendir(settings[SAVE_PATH]);
    }
    struct dirent *entry;
    int palette_number = 0;
    char name[MAX_COUNT_PALETTE + 9] = "palette_%d.svg";
    while ((entry = readdir(palette_dir)) != NULL) {
        if (strncmp(entry->d_name, "palette", 7) == 0)
            palette_number += 1;
    }
    sprintf(name, name, palette_number);

    char *file_name = (char *)calloc(1, (strlen("palette_%d.svg") + MAX_COUNT_PALETTE));

    if (palette_number > 0) {
        strcpy(file_name, name);
    } else {
        strcpy(file_name, "palette.svg");
    }
    settings[SAVE_PATH] = realloc(settings[SAVE_PATH], strlen(settings[SAVE_PATH]) + strlen(file_name) + 1);
    strcat(settings[SAVE_PATH], file_name);

    char *save_path = malloc(strlen(settings[SAVE_PATH]) + 1);
    strcpy(save_path, settings[SAVE_PATH]);

    free(settings);
    free(file_name);
    closedir(palette_dir);
    return save_path;
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
char **get_setting_path() {
    char line[1000];
    char buff[1000];
    bzero(buff, 1000);

    uid_t uid = getuid();
    struct passwd *pw = getpwuid(uid);
    char *path_to_settings_template = "/home/%s/.config/color-hunter/settings.json";
	char *path_to_settings = malloc(10000);
    sprintf(path_to_settings, path_to_settings_template, pw->pw_name);
    FILE *settings_ptr = fopen(path_to_settings, "r");

    char **parametrs = malloc(2 * sizeof(char *));
    int parametrs_step = 0;

    int step = 0;
    bool in_double_quote = false;

    while (fgets(line, 100, settings_ptr)) {
        if (strcmp(line, "{\n") != 0 && strcmp(line, "}\n") != 0)
            for (int i = 0; line[i] != '\0'; i++) {
                if (line[i] == DOUBLE_QUOUTE) {
                    in_double_quote = !in_double_quote;
                    i++;
                    if (in_double_quote == false) {
                        parametrs[parametrs_step] = malloc(strlen(buff) + 1);
                        strcpy(parametrs[parametrs_step++], buff);
                        bzero(buff, 1000);
                        step = 0;
                    }
                }
                if (in_double_quote)
                    buff[step++] = line[i];
            }
    }
    fclose(settings_ptr);
    return parametrs;
}

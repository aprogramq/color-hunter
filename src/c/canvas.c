#include <cairo/cairo-svg.h>
#include <cairo/cairo.h>
#include <pwd.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>

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

void save_palette(int count, char **hex) {
    // for (int i = 0; i < count; i++) {
    //   printf("hex[%d] address: %p, content: %s\n", i, (void *)hex[i],
    //   hex[i]);
    // }
    char path_to_save[100];
    uid_t uid = getuid();
    struct passwd *pw = getpwuid(uid);

    sprintf(path_to_save, "/home/%s/Pictures/palette/example.svg", pw->pw_name);
    cairo_surface_t *surface = cairo_svg_surface_create(path_to_save, 500, 500);

    cairo_t *cr = cairo_create(surface);
    struct RGB colors[count];

    for (int i = 0; i < count; i++) {
        colors[i] = hexToRGB(hex[i]);
    }
    int x_position = 60;
    char *hex_color = "#";
    for (int i = 0; i < count; i++) {
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
        strcpy(hex_color, "#");
    }

    cairo_destroy(cr);
    cairo_surface_destroy(surface);
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

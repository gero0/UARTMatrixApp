#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define MAX_FRAME_SIZE 512

#define MAX_TEXT_LENGTH 255

typedef struct RgbColor {
  uint8_t r;
  uint8_t g;
  uint8_t b;
} RgbColor;

typedef struct Point {
  uint8_t x;
  uint8_t y;
} Point;

int umx_serialize_param_request(unsigned char *buffer);

int umx_serialize_switch_mode(unsigned char *buffer, int mode);

int umx_serialize_write_line(unsigned char *buffer,
                             unsigned char row,
                             const unsigned char *text,
                             unsigned int text_len);

int umx_serialize_set_font(unsigned char *buffer, unsigned char row, unsigned char font);

int umx_serialize_set_color(unsigned char *buffer, unsigned char row, struct RgbColor color);

int umx_serialize_set_animation(unsigned char *buffer,
                                unsigned char row,
                                unsigned char animation,
                                unsigned char speed,
                                unsigned char direction);

int umx_serialize_draw_pixel(unsigned char *buffer, struct Point position, struct RgbColor color);

int umx_serialize_draw_rectangle(unsigned char *buffer,
                                 struct Point point_1,
                                 struct Point point_2,
                                 unsigned char thickness,
                                 struct RgbColor color,
                                 int filled);

int umx_serialize_draw_triangle(unsigned char *buffer,
                                struct Point point_1,
                                struct Point point_2,
                                struct Point point_3,
                                unsigned char thickness,
                                struct RgbColor color,
                                int filled);

int umx_serialize_draw_circle(unsigned char *buffer,
                              struct Point center,
                              unsigned char radius,
                              unsigned char thickness,
                              struct RgbColor color,
                              int filled);

int umx_serialize_draw_row(unsigned char *buffer,
                           unsigned char row,
                           const struct RgbColor *pixels,
                           unsigned int pixels_len);

int umx_serialize_clear(unsigned char *buffer);

int umx_serialize_enable_output(unsigned char *buffer);

int umx_serialize_disable_output(unsigned char *buffer);

int umx_serialize_ping(unsigned char *buffer);

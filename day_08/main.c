#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <unistd.h>

const size_t WIDTH = 25;
const size_t HEIGHT = 6;

#define ASSERT(expr)                        \
  if (!(expr)) {                            \
    fprintf(stderr, "Failed: %s\n", #expr); \
    return 1;                               \
  }

size_t count_char(const char *buf, size_t size, char search_char) {
  size_t count = 0;
  for (const char *end = buf+size; buf < end; ++buf) {
    if (*buf == search_char) {
      count++;
    }
  }

  return count;
}

int main(int argc, char **argv) {
  FILE *file = fopen("./input", "rb");
  ASSERT(file);
  int fd = fileno(file);
  ASSERT(fd > 0);
  struct stat stat_buf;
  ASSERT(fstat(fd, &stat_buf) == 0);
  size_t file_size = stat_buf.st_size;
  ASSERT(file_size > 0);
  file_size -= 1; // Skip final newline
  char *input_image_buf = malloc(file_size);
  ASSERT(input_image_buf);
  ASSERT(fread(input_image_buf, sizeof(char), file_size, file));
  ASSERT(fclose(file) == 0);

  size_t layer_size = WIDTH * HEIGHT;
  size_t num_layers = file_size / layer_size;

  { // Part 1: Find the product of the count of 1s and 2s in the layer with fewest 0s

    size_t candidate_layer_idx = 0;
    size_t candidate_num_zeroes = SIZE_MAX;

    for (size_t layer_idx = 0; layer_idx < num_layers; ++layer_idx) {
      const char *layer_buf = &input_image_buf[layer_idx * layer_size];

      const size_t num_zeroes = count_char(layer_buf, layer_size, '0');
      if (num_zeroes < candidate_num_zeroes) {
        candidate_layer_idx = layer_idx;
        candidate_num_zeroes = num_zeroes;
      }
    }
    const char *candidate_buf = &input_image_buf[candidate_layer_idx * layer_size];
    size_t num_ones = count_char(candidate_buf, layer_size, '1');
    size_t num_twos = count_char(candidate_buf, layer_size, '2');

    printf("Part 1: %zu\n", num_ones * num_twos);
  }

  { // Part 2: Actually decode the image

    char *out_buf = malloc(layer_size);
    ASSERT(out_buf > 0);
    memset(out_buf, ' ', layer_size);

    const char COLOR_BLACK = 0;
    const char COLOR_WHITE = 1;
    const char COLOR_TRANSPARENT = 2;

    for (int layer_idx = num_layers-1; layer_idx >= 0; --layer_idx) {
      const char *layer_buf = &input_image_buf[layer_idx * layer_size];
      for (size_t pixel_idx = 0; pixel_idx < layer_size; ++pixel_idx) {
        const char pixel_ascii = layer_buf[pixel_idx];
        const char pixel_value = pixel_ascii - '0';
        if (pixel_value == COLOR_TRANSPARENT) {
          continue;
        }

        char out_char = ' ';
        if (pixel_value == COLOR_WHITE) {
          out_char = '#';
        }

        out_buf[pixel_idx] = out_char;
      }
    }

    printf("Part 2:\n\n");

    for (size_t row = 0; row < HEIGHT; ++row) {
      const char *row_buf = &out_buf[row * WIDTH];
      printf("%.*s\n", (int) WIDTH, row_buf);
    }

    free(out_buf);
  }

  free(input_image_buf);

  return 0;
}

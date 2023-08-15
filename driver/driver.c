#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/types.h>

#include "pico/stdlib.h"
#include "hardware/pio.h"
#include "hardware/clocks.h"
#include "ws2812.pio.h"

#define IS_RGBW false

#define RGB_VAL(r, g, b) ((((uint32_t) (r) << 8) | ((uint32_t) (g) << 16) | (uint32_t) (b)) << 8u)

#define WS2812_PIN 0

#define WIDTH 12
#define HEIGTH 20

#define FULL 42
#define HALF 120
#define QUARTER 63
#define OUT {0, 0, 0}
#define WHITE {42, 42, 42}
#define RED {42, 0, 0}
#define BLUE {0, 0, 42}
#define GREEN {0, 42, 0}

typedef struct {
    uint8_t red;
    uint8_t green;
    uint8_t blue;
} rgb;

static PIO pio = pio0;
static PIO pioone = pio1;
static int sm[6]; 
static uint offset, offset1;


void drawBitmap(rgb** bitmap) {
	for (int i = 0; i < WIDTH; i++) {
		int index = i >> 1;
		for (int j = HEIGTH-1; j >= 0; j--) {
			if (index > 2) {
				pio_sm_put_blocking(pio1, index-3, RGB_VAL(bitmap[i][j].red, bitmap[i][j].green, bitmap[i][j].blue));
			} else {
				pio_sm_put_blocking(pio0, index, RGB_VAL(bitmap[i][j].red, bitmap[i][j].green, bitmap[i][j].blue));
			}
		} 
	}
}

void shiftBitmap(rgb** bitmap, int first, short step) {
	for (int i = 0; i < WIDTH; i++) {
		rgb first = bitmap[i][0];
		for (int j = 0; j < HEIGTH-1; j++) {
			bitmap[i][j] = bitmap[i][j+1];
		}
		bitmap[i][19] = first;
		/*rgb firstval = bitmap[i][first];
		while(first >= 0 && first < HEIGTH) {
			bitmap[i][first] = bitmap[i][first+step];
			first+=step;
		}*/
	}
}

void shiftOut(rgb** bitmap, int first, short step) {
	for (int i = 0; i < WIDTH; i++) {
		for (int j = 0; j < HEIGTH-1; j++) {
			bitmap[i][j] = bitmap[i][j+1];
		}
		bitmap[i][19].red = 0; bitmap[i][19].green = 0; bitmap[i][19].blue = 0;
	}
}

void shiftRigth(rgb** bitmap) {
	rgb* first = bitmap[WIDTH-1];
	for (int i = WIDTH-1; i > 0; i--) {
		bitmap[i] = bitmap[i-1];
	}
	bitmap[0] = first;
}

void shiftLeft(rgb** bitmap) {
	rgb* first = bitmap[0];
	for (int i = 0; i < WIDTH; i++) {
		bitmap[i] = bitmap[i+1];
	}
	bitmap[WIDTH-1] = first;
}

int main() {
	sm[0] = pio_claim_unused_sm(pio, true);
	sm[1] = pio_claim_unused_sm(pio, true);
	sm[2] = pio_claim_unused_sm(pio, true);
	sm[3] = pio_claim_unused_sm(pioone, true);
	sm[4] = pio_claim_unused_sm(pioone, true);
	sm[5] = pio_claim_unused_sm(pioone, true);
	offset = pio_add_program(pio, &ws2812_program);
	offset1 = pio_add_program(pioone, &ws2812_program);
	ws2812_program_init(pio, sm[0], offset, WS2812_PIN, 800000, IS_RGBW);
	ws2812_program_init(pio, sm[1], offset, WS2812_PIN+1, 800000, IS_RGBW);
	ws2812_program_init(pio, sm[2], offset, WS2812_PIN+2, 800000, IS_RGBW);
	ws2812_program_init(pioone, sm[3], offset1, WS2812_PIN+3, 800000, IS_RGBW);
	ws2812_program_init(pioone, sm[4], offset1, WS2812_PIN+4, 800000, IS_RGBW);
	ws2812_program_init(pioone, sm[5], offset1, WS2812_PIN+5, 800000, IS_RGBW);

   	//pio_sm_put_blocking(pio0, 0, PINK);
    while (true) {
		rgb c1[HEIGTH] = {OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, {0, 0, FULL}, {0, 0, FULL}, {0, 0, FULL}, {0, 0, FULL}, {0, 0, FULL}, {0, 0, FULL}, {0, 0, FULL}, {0, 0, FULL}, OUT, OUT};
		rgb c2[HEIGTH] = {OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, {0, 0, FULL}, {0, 0, FULL}, {0, 0, FULL}, {0, 0, FULL}, {0, 0, FULL}, WHITE, WHITE, WHITE, {0, 0, FULL}, {0, 0, FULL}, OUT};
		rgb c3[HEIGTH] = {OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, {0, 0, FULL}, {0, 0, FULL}, {0, 0, FULL}, {0, 0, FULL}, WHITE, WHITE, GREEN, WHITE , WHITE, {0, 0, FULL}, {0, 0, FULL}};
		rgb c4[HEIGTH] = {OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, {0, 0, FULL}, {0, 0, FULL}, {0, 0, FULL}, WHITE, WHITE, WHITE, WHITE, WHITE, WHITE, {0, 0, FULL}};
		rgb c5[HEIGTH] = {OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, {0, 0, FULL}, {0, 0, FULL}, WHITE, WHITE, WHITE, RED, RED, WHITE, {0, 0, FULL}};
		rgb c6[HEIGTH] = {OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, {0, 0, FULL}, {0, 0, FULL}, WHITE, WHITE, WHITE, WHITE, RED, WHITE, {0, 0, FULL}};
		rgb c7[HEIGTH] = {OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, {0, 0, FULL}, {0, 0, FULL}, WHITE, WHITE, WHITE, RED, RED, WHITE, {0, 0, FULL}};
		rgb c8[HEIGTH] = {OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, {0, 0, FULL}, {0, 0, FULL}, {0, 0, FULL}, WHITE, WHITE, WHITE, WHITE, WHITE, WHITE, {0, 0, FULL}};
		rgb c9[HEIGTH] = {OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, {0, 0, FULL}, {0, 0, FULL}, {0, 0, FULL}, {0, 0, FULL}, WHITE, WHITE, GREEN, WHITE, WHITE, {0, 0, FULL}, {0, 0, FULL}};
		rgb c10[HEIGTH] = {OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, {0, 0, FULL}, {0, 0, FULL}, {0, 0, FULL}, {0, 0, FULL}, {0, 0, FULL}, WHITE, WHITE, WHITE, {0, 0, FULL}, {0, 0, FULL}, OUT};
		rgb c11[HEIGTH] = {OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, {0, 0, FULL}, {0, 0, FULL}, {0, 0, FULL}, {0, 0, FULL}, {0, 0, FULL}, {0, 0, FULL}, {0, 0, FULL}, {0, 0, FULL}, OUT, OUT};
		rgb c12[HEIGTH] = {OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT};
		rgb* bitmap[WIDTH] = {&c1[0], &c2[0], &c3[0], &c4[0], &c5[0], &c6[0], &c7[0], &c8[0], &c9[0], &c10[0], &c11[0], &c12[0]};

		//drawBitmap(&bitmap[0]);
		/*for (int i = 0; i < 9; i++){
			drawBitmap(&bitmap[0]);
			sleep_ms(500);
			shiftBitmap(&bitmap[0], 0, 1);
		}*/
		drawBitmap(&bitmap[0]);
		sleep_ms(500);
		for(int i = 0; i < 10; i++) {
			shiftOut(&bitmap[0], 0, 1);
			shiftRigth(&bitmap[0]);
			drawBitmap(&bitmap[0]);
			sleep_ms(500);
			shiftOut(&bitmap[0], 0, 1);
			shiftLeft(&bitmap[0]);
			drawBitmap(&bitmap[0]);
			sleep_ms(500);
		}
		
    }
}

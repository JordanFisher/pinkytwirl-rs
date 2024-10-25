#ifndef PINKY_TWIRL_H
#define PINKY_TWIRL_H

#include <stdint.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

// Opaque type for the engine
typedef struct PinkyTwirlEngine PinkyTwirlEngine;

// FFI-safe key event structure
typedef struct FFIKeyEvent {
    char* key;
    bool state;  // true for down, false for up
    bool shift;
    bool ctrl;
    bool alt;
    bool meta;
} FFIKeyEvent;

// Create a new engine instance
PinkyTwirlEngine* pinkytwirl_engine_new(const char* config_path);

// Free the engine instance
void pinkytwirl_engine_free(PinkyTwirlEngine* engine);

// Handle a key event
FFIKeyEvent* pinkytwirl_engine_handle_key_event(
    PinkyTwirlEngine* engine,
    uint16_t key_code,
    bool down,
    bool shift,
    bool ctrl,
    bool option,
    bool meta,
    const char* app_name,
    const char* window_name,
    size_t* out_length
);

// Free the key events array
void pinkytwirl_free_key_events(FFIKeyEvent* events, size_t length);

#ifdef __cplusplus
}
#endif

#endif /* PINKY_TWIRL_H */

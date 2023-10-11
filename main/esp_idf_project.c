#include "stdio.h"
#include "esp_rust_component.h"

#include "freertos/FreeRTOS.h"
#include "freertos/task.h"

#include <math.h>


void print_optional_double(const char* label, double value, const char* unit) {
    if (!isnan(value)) {
        printf("- %s: %.1f %s\n", label, value, unit);
    } else {
        printf("- %s: N/A\n", label);
    }
}

void print_optional_coord(const char* label, double coord, const char* dir) {
    if (!isnan(coord)) {
        double deg = floor(coord);
        double min = (coord - deg) * 60.0;
        printf("- %s: %.0f°%.4f' %s\n", label, deg, min, dir);
    } else {
        printf("- %s: N/A\n", label);
    }
}

void print_optional_int(const char* label, int value, const char* unit) {
    if (value != -1) {
        printf("- %s: %d %s\n", label, value, unit);
    } else {
        printf("- %s: N/A\n", label);
    }
}

void print_gga_data(struct CGgaData c_gga_data) {
    printf("GGA Data:\n");
    print_optional_coord("Latitude", c_gga_data.latitude, "N");
    print_optional_coord("Longitude", c_gga_data.longitude, "W");
    print_optional_int("GPS Quality", c_gga_data.fix_type, "(GPS fix)");
    print_optional_int("Number of Satellites", c_gga_data.fix_satellites, "");
    print_optional_double("Horizontal Dilution of Precision", c_gga_data.hdop, "");
    print_optional_double("Altitude", c_gga_data.altitude, "Meters");
    print_optional_double("Height of Geoid above WGS84 Ellipsoid", c_gga_data.geoid_separation, "Meters");
}

void nmea_gga_task(void* param) {
    UBaseType_t remaining_stack = uxTaskGetStackHighWaterMark(NULL);
    printf("Remaining stack space before calling Rust function in nmea_gga_task: %u\n", remaining_stack);

    float altitude = nmea_gga_altitude("$GPGGA,092750.000,5321.6802,N,00630.3372,W,1,8,1.03,61.7,M,55.2,M,,*76");
    printf("NMEA altitude: %f\n", altitude);

    remaining_stack = uxTaskGetStackHighWaterMark(NULL);
    printf("Remaining stack space after calling Rust function in nmea_gga_task: %u\n", remaining_stack);

    struct CGgaData c_gga_data = parse_nmea_gga("$GPGGA,092750.000,5321.6802,N,00630.3372,W,1,8,1.03,61.7,M,55.2,M,,*76");
    print_gga_data(c_gga_data);

    vTaskDelete(NULL);
}

void app_main() {
    UBaseType_t remaining_stack = uxTaskGetStackHighWaterMark(NULL);
    printf("Remaining stack space before calling Rust function in app_main: %u\n", remaining_stack);

    const char* message = hello();
    printf("Message from Rust: %s\n", message);

    printf("Size of NMEA: %lu\n", nmea_size());

    remaining_stack = uxTaskGetStackHighWaterMark(NULL);
    printf("Remaining stack space after calling Rust function in app_main: %u\n", remaining_stack);

    // Create a new task for running nmea_gga with a 12KB stack
    xTaskCreate(nmea_gga_task, "nmea_gga_task", 36240, NULL, 5, NULL);
}

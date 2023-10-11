
#include <stdio.h>
extern const void* hello();

extern uint32_t nmea_size();
extern float nmea_gga_altitude(const char* gga_str);

struct CGgaData {
    int fix_hour;
    int fix_minute;
    int fix_second;
    int fix_type;
    double latitude;
    double longitude;
    int fix_satellites;
    float hdop;
    float altitude;
    float geoid_separation;
};

struct CGgaData parse_nmea_gga(const char* gga_cstr);

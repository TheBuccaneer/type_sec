#define CL_TARGET_OPENCL_VERSION 120
#include <CL/cl.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/time.h>

#define SIZE_MB      500
#define ITERATIONS   10

// Fehlerbehandlung Makro
#define CL_CHECK(call) do { \
cl_int err = call; \
if (err != CL_SUCCESS) { \
    fprintf(stderr, "OpenCL error at %s:%d - Error code: %d\n", __FILE__, __LINE__, err); \
    exit(1); \
} \
} while(0)

double get_time() {
    struct timeval tv;
    gettimeofday(&tv, NULL);
    return tv.tv_sec + tv.tv_usec * 1e-6;
}

int main() {
    size_t size = SIZE_MB * 1024 * 1024;  // Größe in Bytes
    float *h_data = NULL;
    cl_mem d_data;
    cl_platform_id platform;
    cl_device_id device;
    cl_context context;
    cl_command_queue queue;
    cl_int err;
    double total_h2d_s = 0.0, total_d2h_s = 0.0;

    printf("OpenCL Memory Bandwidth Benchmark\n");
    printf("==================================\n");
    printf("Transfer size: %d MB\n", SIZE_MB);
    printf("Iterations: %d\n\n", ITERATIONS);

    // 1. Platform und Device finden
    CL_CHECK(clGetPlatformIDs(1, &platform, NULL));
    CL_CHECK(clGetDeviceIDs(platform, CL_DEVICE_TYPE_GPU, 1, &device, NULL));

    // Device Info ausgeben
    char device_name[256];
    clGetDeviceInfo(device, CL_DEVICE_NAME, sizeof(device_name), device_name, NULL);
    printf("Device: %s\n\n", device_name);

    // 2. Context und Command Queue erstellen
    context = clCreateContext(NULL, 1, &device, NULL, NULL, &err);
    CL_CHECK(err);

    // Moderne API verwenden falls verfügbar, sonst deprecated API
    #ifdef CL_VERSION_2_0
    cl_command_queue_properties props[] = {0};
    queue = clCreateCommandQueueWithProperties(context, device, props, &err);
    #else
    queue = clCreateCommandQueue(context, device, 0, &err);
    #endif
    CL_CHECK(err);

    // 3. Host-Speicher allozieren
    h_data = (float*)malloc(size);
    if (!h_data) {
        fprintf(stderr, "Failed to allocate host memory\n");
        exit(1);
    }

    // 4. Device-Speicher allozieren
    printf("Trying to allocate %zu MB on device...\n", size / (1024*1024));
    d_data = clCreateBuffer(context, CL_MEM_READ_WRITE, size, NULL, &err);
    if (err != CL_SUCCESS) {
        printf("Failed to allocate %zu MB, trying smaller size...\n", size / (1024*1024));
        // Falls 500MB zu viel sind, versuche 100MB
        size = 100 * 1024 * 1024;
        d_data = clCreateBuffer(context, CL_MEM_READ_WRITE, size, NULL, &err);
        if (err == CL_SUCCESS) {
            printf("Successfully allocated %zu MB\n", size / (1024*1024));
        }
    }
    CL_CHECK(err);

    // 5. Host-Daten initialisieren (wichtig für konsistente Messungen)
    for (size_t i = 0; i < size / sizeof(float); i++) {
        h_data[i] = (float)(i % 1000);
    }

    // 6. Host→Device Transfer messen
    printf("Measuring Host→Device transfers...\n");
    for (int i = 0; i < ITERATIONS; i++) {
        double start = get_time();
        CL_CHECK(clEnqueueWriteBuffer(queue, d_data, CL_TRUE, 0, size, h_data, 0, NULL, NULL));
        double end = get_time();
        total_h2d_s += (end - start);
    }

    // 7. Device→Host Transfer messen
    printf("Measuring Device→Host transfers...\n");
    for (int i = 0; i < ITERATIONS; i++) {
        double start = get_time();
        CL_CHECK(clEnqueueReadBuffer(queue, d_data, CL_TRUE, 0, size, h_data, 0, NULL, NULL));
        double end = get_time();
        total_d2h_s += (end - start);
    }

    // 8. Durchschnitt und Bandbreite berechnen (KORREKT!)
    double avg_h2d_s = total_h2d_s / ITERATIONS;
    double avg_d2h_s = total_d2h_s / ITERATIONS;

    double avg_h2d_ms = avg_h2d_s * 1000.0;  // s → ms
    double avg_d2h_ms = avg_d2h_s * 1000.0;  // s → ms

    // Bandbreite in GB/s (1 GB = 1024 MB)
    double actual_mb = size / (1024.0 * 1024.0);
    double bw_h2d_gb = (actual_mb / 1024.0) / avg_h2d_s;
    double bw_d2h_gb = (actual_mb / 1024.0) / avg_d2h_s;

    // Bandbreite in MB/s
    double bw_h2d_mb = actual_mb / avg_h2d_s;
    double bw_d2h_mb = actual_mb / avg_d2h_s;

    // 9. Ergebnisse ausgeben
    printf("\nResults:\n");
    printf("========\n");
    printf("Host→Device:\n");
    printf("  Average time: %.2f ms\n", avg_h2d_ms);
    printf("  Bandwidth:    %.2f GB/s (%.0f MB/s)\n", bw_h2d_gb, bw_h2d_mb);
    printf("\nDevice→Host:\n");
    printf("  Average time: %.2f ms\n", avg_d2h_ms);
    printf("  Bandwidth:    %.2f GB/s (%.0f MB/s)\n", bw_d2h_gb, bw_d2h_mb);

    // 10. Aufräumen
    clReleaseMemObject(d_data);
    clReleaseCommandQueue(queue);
    clReleaseContext(context);
    free(h_data);

    return 0;
}

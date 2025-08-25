#include <stdio.h>
#include <cuda_runtime.h>
#include <stdlib.h>

// Fehlerbehandlung Makro
#define CUDA_CHECK(call) do { \
cudaError_t err = call; \
if (err != cudaSuccess) { \
    fprintf(stderr, "CUDA error at %s:%d - %s\n", __FILE__, __LINE__, cudaGetErrorString(err)); \
    exit(1); \
} \
} while(0)

int main() {
    // 100 MB Test
    size_t size = 100 * 1024 * 1024;
    float *h_data = NULL;
    float *d_data = NULL;
    cudaEvent_t start, stop;

    printf("CUDA Bandwidth Test - 100MB\n");
    printf("============================\n");

    // Device Info
    cudaDeviceProp prop;
    CUDA_CHECK(cudaGetDeviceProperties(&prop, 0));
    printf("Device: %s\n\n", prop.name);

    // Memory allozieren
    CUDA_CHECK(cudaHostAlloc((void**)&h_data, size, cudaHostAllocDefault));
    CUDA_CHECK(cudaMalloc((void**)&d_data, size));

    // Events erstellen
    CUDA_CHECK(cudaEventCreate(&start));
    CUDA_CHECK(cudaEventCreate(&stop));

    // Test data initialisieren
    for (int i = 0; i < size/4; i++) {
        h_data[i] = i % 1000;
    }

    printf("Running 10 iterations...\n\n");

    float total_write_time = 0.0f;
    float total_read_time = 0.0f;

    // 10 Tests
    for (int i = 0; i < 10; i++) {
        // Write test (Host->Device)
        CUDA_CHECK(cudaEventRecord(start));
        CUDA_CHECK(cudaMemcpy(d_data, h_data, size, cudaMemcpyHostToDevice));
        CUDA_CHECK(cudaEventRecord(stop));
        CUDA_CHECK(cudaEventSynchronize(stop));

        float write_ms;
        CUDA_CHECK(cudaEventElapsedTime(&write_ms, start, stop));

        // Read test (Device->Host)
        CUDA_CHECK(cudaEventRecord(start));
        CUDA_CHECK(cudaMemcpy(h_data, d_data, size, cudaMemcpyDeviceToHost));
        CUDA_CHECK(cudaEventRecord(stop));
        CUDA_CHECK(cudaEventSynchronize(stop));

        float read_ms;
        CUDA_CHECK(cudaEventElapsedTime(&read_ms, start, stop));

        total_write_time += write_ms;
        total_read_time += read_ms;

        // Bandbreite berechnen
        float write_bw = (100.0f / 1024.0f) / (write_ms / 1000.0f);  // GB/s
        float read_bw = (100.0f / 1024.0f) / (read_ms / 1000.0f);    // GB/s

        printf("Test %2d: Write %.2f GB/s, Read %.2f GB/s\n", i+1, write_bw, read_bw);
    }

    // Durchschnitt berechnen
    float avg_write_bw = (100.0f / 1024.0f) / ((total_write_time / 10.0f) / 1000.0f);
    float avg_read_bw = (100.0f / 1024.0f) / ((total_read_time / 10.0f) / 1000.0f);

    printf("\nAverage Results:\n");
    printf("Write Bandwidth: %.2f GB/s\n", avg_write_bw);
    printf("Read Bandwidth:  %.2f GB/s\n", avg_read_bw);

    // Cleanup
    CUDA_CHECK(cudaEventDestroy(start));
    CUDA_CHECK(cudaEventDestroy(stop));
    CUDA_CHECK(cudaFree(d_data));
    CUDA_CHECK(cudaFreeHost(h_data));

    return 0;
}

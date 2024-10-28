__kernel void fill(__global uchar3* buffer, ulong2 p0, ulong2 p1, ulong2 p2, ulong2 size, uchar3 color) {
    size_t gid = get_global_id(0);
    ulong x = gid % size.x;
    ulong y = gid / size.x;

    long ax = p1.x - p0.x;
    long ay = p1.y - p0.y;
    long bx = p2.x - p0.x;
    long by = p2.y - p0.y;
    long cx = x - p0.x;
    long cy = y - p0.y;

    long denom = ax * by - ay * bx;
    if (denom == 0) return; 

    float u = (float)(cx * by - cy * bx) / denom;
    float v = (float)(ax * cy - ay * cx) / denom;
    float w = 1.0f - u - v;

    if (u >= 0 && v >= 0 && w >= 0) {
        buffer[gid] = color;
    }
}
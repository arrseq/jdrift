__kernel void fill(__global uchar3* buffer, ulong2 size, uchar glow) {
    size_t gid = get_global_id(0);

    uchar3 color = 0;

    ulong x = gid % size.x;
    ulong y = gid / size.x;

    ulong2 p0 = (100, 100);
    ulong2 p1 = (200, 200);
    buffer[gid] = (uchar3) (0, 0, 0);

    if (!(x >= p0.x && x <= p1.x)) { return; }
    if (!(y >= p0.y && y <= p1.y)) { return; }

    if (glow) {
        buffer[gid] = (uchar3) (255, 200, 30);
        return;
    }
    buffer[gid] = (uchar3) (255, 0, 0);
}
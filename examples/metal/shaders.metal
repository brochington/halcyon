#include <metal_stdlib>

using namespace metal;

typedef struct {
  float x, y, r, g, b;
} vertex_t;

struct ColorInOut {
  float4 position [[position]];
  float4 color;
};

vertex ColorInOut triangle_vertex(device vertex_t* vertex_array [[buffer(0)]], uint32_t vid [[vertex_id]]) 
{
  ColorInOut out;
  auto device const &v = vertex_array[vid];
  out.position = float4(v.x, v.y, 0.0, 1.0);
  out.color = float4(v.r, v.g, v.b, 1.0);

  return out;
}

fragment float4 triangle_fragment(ColorInOut in [[stage_in]])
{
  return in.color;
}
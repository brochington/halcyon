#include <metal_stdlib>

using namespace metal;

typedef struct {
  // float x, y, r, g, b;
  float x, y, z, mode, r, g, b, a, tx, ty;
} vertex_t;

struct ColorInOut {
  float4 position [[position]];
  float4 color;
  float2 texCoord;
  float mode;
};

vertex ColorInOut triangle_vertex(
  device vertex_t* vertex_array [[buffer(0)]],
  uint32_t vid [[vertex_id]]
) 
{
  ColorInOut out;
  auto device const &v = vertex_array[vid];
  out.position = float4(v.x, v.y, 0.0, 1.0);
  out.mode = v.mode;
  out.color = float4(v.r, v.g, v.b, 1.0);
  out.texCoord = float2(v.tx, v.ty);

  return out;
}

fragment float4 triangle_fragment(
    ColorInOut in[[stage_in]],
    texture2d<half> tex [[texture(0)]]
)
{
  if (in.mode == 0.0) {
    return in.color;
  } else {
     constexpr sampler textureSampler(mag_filter::linear, min_filter::linear);

     const half4 colorSample = tex.sample(textureSampler, in.texCoord);

     return float4(colorSample);
  };
}
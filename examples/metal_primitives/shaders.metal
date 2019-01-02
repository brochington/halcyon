#include <metal_stdlib>

using namespace metal;

typedef struct {
  float x, y, r, g, b;
  // float x, y, r, g, b, tx, ty;
} vertex_t;

struct ColorInOut {
  float4 position [[position]];
  float4 color;
  // float2 textureCoordinate;
  // float x_;
  // float y_;
};

vertex ColorInOut triangle_vertex(
  device vertex_t* vertex_array [[buffer(0)]],
  uint32_t vid [[vertex_id]]
) 
{
  ColorInOut out;
  auto device const &v = vertex_array[vid];
  out.position = float4(v.x, v.y, 0.0, 1.0);
  out.color = float4(v.r, v.g, v.b, 1.0);
  // out.x_ = v.tx;
  // out.y_ = v.ty;

      return out;
}

fragment float4 triangle_fragment(
    ColorInOut in[[stage_in]])
    // texture2d<half> tex [[texture(0)]])
{
  // float2 coord;
  // constexpr sampler textureSampler(mag_filter::linear,
  //                                  min_filter::linear);
  // coord.x = in.x_;
  // coord.y = in.y_;
  // const half4 colorSample = tex.sample(textureSampler, coord);

  // return tex.sample(textureSampler, coord);
  // return float4(colorSample);
  return in.color;
}
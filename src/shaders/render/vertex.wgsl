#define_import_path bevy_terrain::vertex

#import bevy_terrain::types::{Blend, Coordinate, WorldCoordinate}
#import bevy_terrain::bindings::{terrain, terrain_view, approximate_height}
#import bevy_terrain::functions::{compute_coordinate, compute_world_coordinate, correct_world_coordinate, apply_height, lookup_tile, morph_coordinate, compute_blend}
#import bevy_terrain::attachments::sample_height
#import bevy_pbr::mesh_view_bindings::view
#import bevy_pbr::view_transformations::position_world_to_clip

struct VertexInput {
    @builtin(vertex_index) vertex_index: u32,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tile_index: u32,
    @location(1) tile_uv: vec2<f32>,
    @location(2) height: f32,
}

struct VertexInfo {
    tile_index: u32,
    coordinate: Coordinate,
    world_coordinate: WorldCoordinate,
    blend: Blend,
}

fn vertex_info(input: VertexInput) -> VertexInfo {
    let approximate_coordinate    = compute_coordinate(input.vertex_index);
    let approximate_view_distance = compute_world_coordinate(approximate_coordinate, approximate_height).view_distance;

    var info: VertexInfo;
    info.tile_index       = input.vertex_index / terrain_view.vertices_per_tile;
    info.coordinate       = morph_coordinate(approximate_coordinate, approximate_view_distance);
    info.world_coordinate = correct_world_coordinate(info.coordinate, approximate_view_distance);
    info.blend            = compute_blend(info.world_coordinate.view_distance);
    return info;
}

fn vertex_output(info: ptr<function, VertexInfo>, height: f32) -> VertexOutput {
    var output: VertexOutput;
    output.clip_position = position_world_to_clip(apply_height((*info).world_coordinate, height));
    output.tile_index    = (*info).tile_index;
    output.tile_uv       = (*info).coordinate.uv;
    output.height        = height;
    return output;
}

@vertex
fn vertex(input: VertexInput) -> VertexOutput {
    var info = vertex_info(input);

    let tile   = lookup_tile(info.coordinate, info.blend);
    // Sample height and normalize it between min/max height
    var height = sample_height(tile);
    
    // First normalize the raw height value to 0-1 range
    height = (height / terrain.height_scale - terrain.min_height) / (terrain.max_height - terrain.min_height);
    
    // Then apply the terrain scale for final displacement
    height = height * terrain.scale * 0.005;

    return vertex_output(&info, height);
}

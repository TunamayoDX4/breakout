// バーテックスシェーダ

// バーテックスシェーダの入力
struct VSInput {
    @location(0) position: vec4<f32>, 
    @location(1) color: vec4<f32>, 
}

// バーテックスシェーダの出力
struct VSOutput {
    @builtin(position) clip_position: vec4<f32>, 
    @location(0) color: vec4<f32>, 
}

// インスタンスの入力
struct Instance {
    @location(5) position: vec2<f32>, 
    @location(6) size: vec2<f32>, 
    @location(7) rotation: vec2<f32>, 
    @location(8) color: vec4<f32>, 
}

// カメラ行列
struct CameraMat {
    view: mat4x4<f32>, 
}
@group(0) @binding(0) var<uniform> camera: CameraMat;

@vertex
fn vs_main(
    model: VSInput, 
    instance: Instance, 
) -> VSOutput {
    var out: VSOutput;

    // 色の処理
    out.color = model.color * instance.color;

    // 座標変換
    var size = vec2<f32>(
        instance.size.x * model.position.x, 
        instance.size.y * model.position.y,  
    );
    var roted_pos = vec3<f32>(
        size.x * instance.rotation.x - size.y * instance.rotation.y, 
        size.x * instance.rotation.y + size.y * instance.rotation.x, 
        1., 
    );
    var pos = vec3<f32>(
        (instance.position.x - camera.view[3][0]) + roted_pos.x - 1., 
        (instance.position.y - camera.view[3][1]) + roted_pos.y - 1., 
        1., 
    );
    var view = mat3x3<f32>(
        camera.view[0][0], camera.view[1][0], camera.view[2][0], 
        camera.view[0][1], camera.view[1][1], camera.view[2][1], 
        camera.view[0][2], camera.view[1][2], camera.view[2][2], 
    );
    var proj_pos = view * pos;
    out.clip_position = vec4<f32>(
        (proj_pos.x * 1.) * 2., 
        (proj_pos.y * 1.) * 2., 
        model.position.z,  
        model.position.w, 
    );
    return out;
}

// フラグメントシェーダ

@fragment
fn fs_main(in: VSOutput) -> @location(0) vec4<f32> {
    return in.color;
}